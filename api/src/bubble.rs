use crate::models::*;
use crate::schema::stickerpack::dsl::*;
use diesel::{pg::PgConnection, prelude::*};
use teloxide::{prelude::*, requests::RequestWithFile, types::*};

fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn bubble(cs: UpdateWithCx<Message>) -> ResponseResult<Message> {
    use crate::schema::stickerpack::dsl::*;

    let connection = establish_connection();

    let data_base_pack = stickerpack
        .filter(chat_id.eq(cs.chat_id()))
        .load::<StickerPack>(&connection)
        .expect("Error loading sticker pack")
        .into_iter()
        .next();

    let pack = match data_base_pack {
        Some(p) => p,
        None => create_sticker_pack(&cs, connection).await?,
    };

    cs.answer_sticker(InputFile::FileId(
        (&cs.bot.get_sticker_set(pack.name.clone()).send().await?.stickers[0]).file_id.clone(),
    ))
    .send()
    .await
    .expect("No file manipulations expected")
}

async fn create_sticker_pack(
    cs: &UpdateWithCx<Message>,
    connection: PgConnection,
) -> Result<StickerPack, RequestError> {
    // TODO config somehow
    let bot_name = "PullPartyTestBot";

    let message_chat_id = cs.chat_id();
    let chat_id_string = message_chat_id.to_string().replace("-", "minus");

    let sender_id = cs.update.from().map(|x| x.id).expect("User is not specified");
    let sticker_pack_name = format!("{}_by_{}", chat_id_string, bot_name);
    let chat = &cs.update.chat;
    let chat_name = match chat.kind.to_owned() {
        ChatKind::Public(public_chat) => public_chat.title.unwrap_or(String::from("")),
        ChatKind::Private(private_chat) => private_chat.username.unwrap_or(String::from("")),
        _ => panic!("Unexpected chat kind met"),
    };

    let sticker_pack_title =
        format!("{}Bubbles", if chat_name.is_empty() { String::from("") } else { chat_name + " " });

    // TODO configuration
    let logo_path: std::path::PathBuf = ["..", "assets", "title-512.png"].iter().collect();

    log::info!(
        "Creating sticker pack \"{}\" (https://t.me/addstickers/{}) with {} owner",
        sticker_pack_title,
        sticker_pack_name,
        sender_id
    );

    cs.bot
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

    if let Err(error) = update_sticker_pack_cover(cs, &sticker_pack_name, sender_id).await {
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

async fn update_sticker_pack_cover(
    cs: &UpdateWithCx<Message>,
    sticker_pack_name: &str,
    sender_id: i32,
) -> anyhow::Result<()> {
    log::info!("Updating sticker pack cover ({} of {})", sticker_pack_name, sender_id);

    if let Some(chat_photo) = cs.bot.get_chat(cs.chat_id()).send().await?.photo {
        let File { file_path, .. } = cs.bot.get_file(chat_photo.small_file_id).send().await?;

        // TODO adequate temporary file
        let tmp_file_path: std::path::PathBuf = ["/", "tmp", "tmp.jpg"].iter().collect();
        let mut file = tokio::fs::File::create(tmp_file_path.clone()).await?;

        cs.bot.download_file(&file_path, &mut file).await?;

        image::open(tmp_file_path.clone())?
            .resize(100, 100, image::imageops::FilterType::Triangle)
            .save(tmp_file_path.clone())?;

        cs.bot
            .set_sticker_set_thumb(sticker_pack_name, sender_id)
            .thumb(InputFile::File(tmp_file_path))
            .send()
            .await?;
    }

    Ok(())
}
