use crate::models::*;
use crate::schema::stickerpack::dsl::*;
use crate::settings::Settings;
use crate::Clown;

use anyhow::*;
use diesel::{pg::PgConnection, prelude::*};
use image::io::Reader as ImageReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use teloxide::{net::Download, prelude::*, requests::Request, types::*};
use thirtyfour::prelude::*;

pub async fn bubble<R: Requester>(
    bot: Bot,
    context: &UpdateWithCx<R, Message>,
    settings: Arc<Settings>,
    connection: Arc<Mutex<PgConnection>>,
) -> Result<()> {
    use crate::schema::stickerpack::dsl::*;

    let text = context
        .update
        .reply_to_message()
        .and_then(|m| m.text())
        .ok_or(anyhow!("Please reply to a bubble text message"))?;

    // TODO user can delete stickerpack, what to do then?
    let data_base_pack = stickerpack
        .filter(chat_id.eq(context.chat_id()))
        .load::<StickerPack>(&*connection.lock().unwrap())
        .expect("Error loading sticker pack")
        .into_iter()
        .next();

    let pack = match data_base_pack {
        Some(p) => p,
        None => create_sticker_pack(bot.clone(), &context, settings.clone(), connection).await?,
    };

    // TODO TEMP: answer w/ a made one
    let first_sticker = InputFile::FileId(
        (&context.requester.get_sticker_set(pack.name.clone()).send().await.clown()?.stickers[0])
            .file_id
            .clone(),
    );
    context.answer_sticker(first_sticker).send().await.clown()?;

    let bubble = render_bubble(&text, settings).await?;

    context.answer_photo(bubble).send().await.clown()?;

    // match context
    //     .bot
    //     .add_sticker_to_set(
    //         pack.user_
    // id as i32,
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
    //         .clown()?;
    //     }
    //     Err(err) => {
    //         context.answer("Failed to add a sticker to your sticker pack").send().await?;
    //     }
    // }

    Ok(())
}

async fn create_sticker_pack<R: Requester>(
    bot: Bot,
    context: &UpdateWithCx<R, Message>,
    settings: Arc<Settings>,
    connection: Arc<Mutex<PgConnection>>,
) -> Result<StickerPack> {
    let message_chat_id = context.chat_id();

    let sender_id = context.update.from().map(|x| x.id).expect("User is not specified");
    let sticker_pack_name = format!(
        "{}_by_{}",
        message_chat_id.to_string().replace("-", "minus"),
        &settings.teloxide.name
    );

    let chat = &context.update.chat;

    let chat_name = match chat.kind.to_owned() {
        ChatKind::Public(public_chat) => public_chat.title,
        ChatKind::Private(private_chat) => private_chat.username,
    };
    let sticker_pack_title = chat_name.map(|x| x + " ").unwrap_or(String::from("")) + "Bubbles";

    log::info!(
        "Creating sticker pack \"{}\" (https://t.me/addstickers/{}) with {} owner",
        sticker_pack_title,
        sticker_pack_name,
        sender_id
    );

    context
        .requester
        .create_new_sticker_set(
            sender_id,
            sticker_pack_name.clone(),
            sticker_pack_title,
            InputSticker::Png(InputFile::File(PathBuf::from(&settings.pack.logo))),
            "💭",
        )
        .send()
        .await
        .clown()?;

    if let Err(error) =
        update_sticker_pack_cover(bot.clone(), context, &sticker_pack_name, sender_id).await
    {
        log::error!("Failed to update sticker pack cover: {}", error);
    }

    diesel::insert_into(stickerpack)
        .values(&NewStickerPack {
            chat_id: &message_chat_id,
            user_id: &(sender_id as i64),
            name: &sticker_pack_name,
        })
        .get_result::<StickerPack>(&*connection.lock().unwrap())
        .expect("Error saving post");

    Ok(StickerPack {
        chat_id: message_chat_id,
        user_id: sender_id as i64,
        name: sticker_pack_name.clone(),
    })
}

async fn update_sticker_pack_cover<R: Requester>(
    bot: Bot,
    context: &UpdateWithCx<R, Message>,
    sticker_pack_name: &str,
    sender_id: i64,
) -> Result<()> {
    log::info!("Updating sticker pack cover ({} of {})", sticker_pack_name, sender_id);

    let chat_photo = match context.requester.get_chat(context.chat_id()).send().await.clown()?.photo
    {
        Some(s) => s,
        None => {
            log::info!("No chat photo found in {}", sender_id);
            return Ok(());
        }
    };

    let File { file_path, .. } =
        context.requester.get_file(chat_photo.small_file_id).send().await.clown()?;

    let raw_data = bot
        .download_file_stream(&file_path)
        .collect::<Vec<Result<bytes::Bytes, _>>>()
        .await
        .into_iter()
        .collect::<Result<Vec<bytes::Bytes>, _>>()
        .map(|v| v.into_iter().flat_map(|b| b.into_iter()).collect::<bytes::Bytes>())?;

    let transformed_data = ImageReader::new(std::io::Cursor::new(raw_data))
        .with_guessed_format()?
        .decode()?
        .resize(100, 100, image::imageops::FilterType::Triangle)
        .to_bytes();

    context
        .requester
        .set_sticker_set_thumb(sticker_pack_name, sender_id)
        .thumb(InputFile::memory(file_path, transformed_data))
        .send()
        .await
        .clown()?;

    log::info!("Sticker pack cover updated successfully ({} of {})", sticker_pack_name, sender_id);

    Ok(())
}

async fn render_bubble(_message: &str, settings: Arc<Settings>) -> Result<InputFile> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new(&settings.selenium.server, &caps).await?;

    driver.get(&settings.selenium.url).await?;
    let screenshot =
        driver.find_element(By::ClassName("L3eUgb")).await?.screenshot_as_png().await?;
    Ok(InputFile::memory("bubble.png", screenshot))
}
