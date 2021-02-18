#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, prelude::*};
use teloxide::{
    prelude::*, requests::RequestWithFile, types::InputFile::FileId, utils::command::BotCommand,
};

pub mod models;
pub mod schema;

use self::models::StickerPack;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Available commands")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "create a bubble sticker")]
    Bubble,
}

async fn answer(cs: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    // TODO logging?

    match command {
        Command::Help => cs.answer(Command::descriptions()).send().await?,
        Command::Bubble => bubble(cs).await?,
    };

    Ok(())
}

async fn bubble(cs: UpdateWithCx<Message>) -> Result<Message, RequestError> {
    use self::schema::stickerpack::dsl::*;

    let connection = establish_connection();
    let result = stickerpack
        .filter(chat_id.eq(cs.chat_id()))
        .load::<StickerPack>(&connection)
        .expect("Error loading sticker pack");

    assert!(result.len() < 2);

    let pack;

    if !result.is_empty() {
        pack = &result[0];
    } else {
        // TODO creating
        pack = &result[0];
    }

    cs.answer_sticker(FileId(
        (&cs.bot.get_sticker_set(pack.name.clone()).send().await?.stickers[0]).file_id.clone(),
    ))
    .send()
    .await
    .expect("No file manipulations expected")
}

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    // TODO config somehow
    let bot_name = "PullPartyTestBot";

    teloxide::commands_repl(bot, &bot_name, answer).await;
}
