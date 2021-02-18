#[derive(Queryable)]
pub struct StickerPack {
    pub chat_id: i64,
    pub user_id: i64,
    pub name: String,
}
