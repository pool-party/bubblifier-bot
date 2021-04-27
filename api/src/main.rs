#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, prelude::*};
use std::result::Result;
use std::sync::{Arc, Mutex};
use teloxide::{dispatching::*, prelude::*, utils::command::BotCommand};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub mod bubble;
pub mod models;
pub mod schema;
pub mod settings;
pub mod utils;

use self::settings::Settings;
use self::utils::Clown;

fn establish_connection(url: String) -> Arc<Mutex<PgConnection>> {
    Arc::new(Mutex::new(
        PgConnection::establish(&url).expect(&format!("Error connecting to {}", url)),
    ))
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Available commands:\n")]
enum Command {
    #[command(description = "start bubblifying")]
    Start,
    #[command(description = "display this text")]
    Help,
    #[command(description = "create a bubble sticker")]
    Bubble,
}

async fn answer<R: Requester>(
    bot: Bot,
    context: UpdateWithCx<R, Message>,
    command: Command,
    settings: Arc<Settings>,
    connection: Arc<Mutex<PgConnection>>,
) -> anyhow::Result<()> {
    log::info!("Processing {:?} from {}", context.update.text(), context.chat_id());

    match command {
        Command::Start => {
            context.answer("bot started").send().await.clown()?;
        }
        Command::Help => {
            context.answer(Command::descriptions()).send().await.clown()?;
        }
        Command::Bubble => {
            if let Err(err) = bubble::bubble(bot.clone(), &context, settings, connection).await {
                log::error!("{}", err);
                context.answer(format!("Error: {}", err)).send().await.clown()?;
            }
        }
    };

    Ok(())
}

async fn run() {
    dotenv::dotenv().ok();
    let settings = match Settings::new() {
        Ok(s) => Arc::new(s),
        Err(r) => panic!("{}", r),
    };

    teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
    log::info!("Starting bot...");

    let bot = teloxide::Bot::new(settings.teloxide.token.clone());

    let handler = Arc::new(answer);
    let connection = establish_connection(settings.database.url.clone());
    let cloned_bot = bot.clone();

    Dispatcher::new(bot.clone())
        .messages_handler(move |rx: DispatcherHandlerRx<_, Message>| {
            UnboundedReceiverStream::new(rx)
                .commands(settings.teloxide.name.clone())
                .for_each_concurrent(None, move |(message, command)| {
                    let handler = handler.clone();
                    let connection_clone = connection.clone();
                    let settings_clone = settings.clone();
                    let bot_clone = cloned_bot.clone();

                    async move {
                        handler(bot_clone, message, command, settings_clone, connection_clone)
                            .await
                            .log_on_error()
                            .await;
                    }
                })
        })
        .dispatch_with_listener(
            update_listeners::polling_default(bot),
            teloxide::error_handlers::LoggingErrorHandler::with_custom_text(
                "An error from the update listener",
            ),
        )
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
