use crate::models::*;
use crate::schema::stickerpack::dsl::*;
use anyhow::*;
use diesel::{pg::PgConnection, prelude::*};
use teloxide::{prelude::*, requests::RequestWithFile, types::*};
use thirtyfour::prelude::*;

fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn bubble(context: UpdateWithCx<Message>) -> Result<()> {
    use crate::schema::stickerpack::dsl::*;

    let text = context
        .update
        .reply_to_message()
        .ok_or(anyhow!("Please reply to a bubble message"))?
        .text();

    // TODO establish a connection only once
    let connection = establish_connection();

    let data_base_pack = stickerpack
        .filter(chat_id.eq(context.chat_id()))
        .load::<StickerPack>(&connection)
        .expect("Error loading sticker pack")
        .into_iter()
        .next();

    let pack = match data_base_pack {
        Some(p) => p,
        None => create_sticker_pack(&context, connection).await?,
    };

    // TODO TEMP: answer w/ a made one
    let first_sticker = InputFile::FileId(
        (&context.bot.get_sticker_set(pack.name.clone()).send().await?.stickers[0]).file_id.clone(),
    );

    // TODO TEMP: answer w/ a made one
    context.answer_sticker(first_sticker).send().await.expect("No file manipulations expected")?;

    // let bubble = render_bubble(&text).await?;

    // match context
    //     .bot
    //     .add_sticker_to_set(
    //         pack.user_id as i32,
    //         pack.name,
    //         StickerType::Png(bubble),
    //         "💭", // TODO smth better
    //     )
    //     .send()
    //     .await
    // {
    //     Ok(_) => {
    //         // TODO TEMP: answer w/ a made one
    //         context.answer_sticker(InputFile::FileId(
    //             (&context.bot.get_sticker_set(pack.name.clone()).send().await?.stickers[0])
    //                 .file_id
    //                 .clone(),
    //         ))
    //         .send()
    //         .await
    //         .expect("No file manipulations expected");
    //     }
    //     Err(err) => {
    //         context.answer("Failed to add a sticker to your sticker pack").send().await?;
    //     }
    // }

    Ok(())
}

async fn create_sticker_pack(
    context: &UpdateWithCx<Message>,
    connection: PgConnection,
) -> Result<StickerPack> {
    // TODO config
    let bot_name = "PullPartyTestBot";

    let message_chat_id = context.chat_id();

    let sender_id = context.update.from().map(|x| x.id).expect("User is not specified");
    let sticker_pack_name =
        format!("{}_by_{}", message_chat_id.to_string().replace("-", "minus"), bot_name);
    let chat = &context.update.chat;
    let chat_name = match chat.kind.to_owned() {
        ChatKind::Public(public_chat) => public_chat.title,
        ChatKind::Private(private_chat) => private_chat.username,
        _ => panic!("Unexpected chat kind met"),
    }
    .map(|x| x + " ")
    .unwrap_or(String::from(""));

    let sticker_pack_title = chat_name + "Bubbles";

    // TODO config
    let logo_path: std::path::PathBuf = ["..", "assets", "title-512.png"].iter().collect();

    log::info!(
        "Creating sticker pack \"{}\" (https://t.me/addstickers/{}) with {} owner",
        sticker_pack_title,
        sticker_pack_name,
        sender_id
    );

    context
        .bot
        .create_new_sticker_set(
            sender_id,
            sticker_pack_name.clone(),
            sticker_pack_title,
            StickerType::Png(InputFile::File(logo_path)),
            "💭",
        )
        .send()
        .await
        .unwrap()?;

    if let Err(error) = update_sticker_pack_cover(context, &sticker_pack_name, sender_id).await {
        log::error!("Failed to update sticker pack cover: {}", error);
    }

    diesel::insert_into(stickerpack)
        .values(&NewStickerPack {
            chat_id: &message_chat_id,
            user_id: &(sender_id as i64),
            name: &sticker_pack_name,
        })
        .get_result::<StickerPack>(&connection)
        .expect("Error saving post");

    Ok(StickerPack {
        chat_id: message_chat_id,
        user_id: sender_id as i64,
        name: sticker_pack_name.clone(),
    })
}

// FIXME this doesn't work wtf
async fn update_sticker_pack_cover(
    context: &UpdateWithCx<Message>,
    sticker_pack_name: &str,
    sender_id: i32,
) -> Result<()> {
    log::info!("Updating sticker pack cover ({} of {})", sticker_pack_name, sender_id);

    if let Some(chat_photo) = context.bot.get_chat(context.chat_id()).send().await?.photo {
        let File { file_path, .. } = context.bot.get_file(chat_photo.small_file_id).send().await?;

        let mut tmp_file_path = std::env::temp_dir();
        tmp_file_path.push(format!("{}.jpg", uuid::Uuid::new_v4()));
        log::info!("Creating temporary file: {:?}", tmp_file_path.to_str());

        let mut file = tokio::fs::File::create(tmp_file_path.clone()).await?;

        context.bot.download_file(&file_path, &mut file).await?;

        image::open(tmp_file_path.clone())?
            .resize(100, 100, image::imageops::FilterType::Triangle)
            .save(tmp_file_path.clone())?;

        context
            .bot
            .set_sticker_set_thumb(sticker_pack_name, sender_id)
            .thumb(InputFile::File(tmp_file_path))
            .send()
            .await?;

        log::info!(
            "Sticker pack cover updated successfully ({} of {})",
            sticker_pack_name,
            sender_id
        );
    } else {
        log::info!("No chat photo found in {}", sender_id);
    }

    Ok(())
}

async fn render_bubble(message: &str) -> Result<InputFile> {
    let caps = DesiredCapabilities::chrome();
    // TODO config
    // TODO not to forget to launch this fucking server
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    // TODO config
    // TODO proper url w/ message
    driver.get("https://www.rust-lang.org").await?;
    let screenshot =
        driver.find_element(By::ClassName("message")).await?.screenshot_as_png().await?;
    Ok(InputFile::memory("bubble.png", screenshot))
}
