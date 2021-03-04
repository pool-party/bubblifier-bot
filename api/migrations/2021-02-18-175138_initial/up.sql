create table StickerPack (
  chat_id bigint primary key not null,
  user_id bigint not null,
  name varchar(150) not null
);
