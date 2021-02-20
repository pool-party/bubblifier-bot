#[macro_use]
extern crate diesel;

// use diesel::{pg::PgConnection, prelude::*};
// use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommand};

pub mod bubble;
pub mod models;
pub mod schema;
pub mod settings;

// use self::settings::Settings;

// fn establish_connection() -> Arc<Mutex<PgConnection>> {
//     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     Arc::new(Mutex::new(
//         PgConnection::establish(&database_url)
//             .expect(&format!("Error connecting to {}", database_url)),
//     ))
// }

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Available commands")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "create a bubble sticker")]
    Bubble,
}

async fn answer(cs: UpdateWithCx<Message>, command: Command) -> anyhow::Result<()> {
    log::info!("Processing {:?} from {}", cs.update.text(), cs.chat_id());

    match command {
        Command::Help => {
            cs.answer(Command::descriptions()).send().await?;
        }
        Command::Bubble => {
            bubble::bubble(cs).await?;
        }
    };

    Ok(())
}

async fn run() {
    dotenv::dotenv().ok();
    // let settings = match Settings::new() {
    //     Ok(s) => Arc::new(tokio::sync::Mutex::new(s)),
    //     Err(r) => panic!(r),
    // };

    teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
    log::info!("Starting bot...");

    // TODO what the fuck, why can't I move it into config?
    // let bot = BotBuilder::new().token(settings.teloxide.token).build();
    let bot = Bot::from_env();

    // TODO what the fuck, why can't I move it into config?
    let bot_name = "PullPartyTestBot";
    // let connection = establish_connection();

    teloxide::commands_repl(bot, bot_name, answer).await;
    // teloxide::commands_repl(bot, bot_name, |cs, command| answer(cs, command, connection)).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
