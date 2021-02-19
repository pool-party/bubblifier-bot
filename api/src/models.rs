use crate::schema::stickerpack;

#[derive(Queryable)]
pub struct StickerPack {
    pub chat_id: i64,
    pub user_id: i64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "stickerpack"]
pub struct NewStickerPack<'a> {
    pub chat_id: &'a i64,
    pub user_id: &'a i64,
    pub name: &'a String,
}
