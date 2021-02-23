#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, prelude::*};
use std::sync::{Arc, Mutex};
use teloxide::{dispatching::*, prelude::*, utils::command::BotCommand};

pub mod bubble;
pub mod models;
pub mod schema;
pub mod settings;

use self::settings::Settings;

fn establish_connection() -> Arc<Mutex<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Mutex::new(
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)),
    ))
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Available commands")]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "create a bubble sticker")]
    Bubble,
}

async fn answer(
    context: UpdateWithCx<Message>,
    command: Command,
    settings: Arc<Settings>,
    connection: Arc<Mutex<PgConnection>>,
) -> anyhow::Result<()> {
    log::info!("Processing {:?} from {}", context.update.text(), context.chat_id());

    match command {
        Command::Help => {
            context.answer(Command::descriptions()).send().await?;
        }
        Command::Bubble => {
            bubble::bubble(context, settings, connection).await?;
        }
    };

    Ok(())
}

async fn run() {
    dotenv::dotenv().ok();
    let settings = match Settings::new() {
        Ok(s) => Arc::new(s),
        Err(r) => panic!(r),
    };

    teloxide::enable_logging_with_filter!(log::LevelFilter::Debug);
    log::info!("Starting bot...");

    let bot = teloxide::BotBuilder::new().token(settings.teloxide.token.clone()).build();

    let handler = Arc::new(answer);
    let connection = establish_connection();
    let cloned_bot = bot.clone();

    Dispatcher::new(bot)
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.commands(settings.teloxide.name.clone()).for_each_concurrent(
                None,
                move |(context, command)| {
                    let handler = handler.clone();

                    let connection_clone = connection.clone();
                    let settings_clone = settings.clone();

                    async move {
                        handler(context, command, settings_clone, connection_clone)
                            .await
                            .log_on_error()
                            .await;
                    }
                },
            )
        })
        .dispatch_with_listener(
            update_listeners::polling_default(cloned_bot),
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
