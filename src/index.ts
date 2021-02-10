import { Telegraf } from "telegraf";
import { PrismaClient, StickerPack } from "@prisma/client";
import chrome from "selenium-webdriver/chrome";
import chromedriver from "chromedriver";
import got from "got";
import sharp from "sharp";
import { Builder, By, WebElement } from "selenium-webdriver";
import { resolve } from "path";
import { writeFile } from "fs";
import { BotCommand } from "telegraf/src/telegram-types";

const token = process.env.BOT_TOKEN;
if (!token) {
  throw new Error("BOT_TOKEN must be provided");
}

const botName = process.env.BOT_NAME;
if (!botName) {
  throw new Error("BOT_NAME must be provided");
}

const bot = new Telegraf(token);

const prisma = new PrismaClient({
  log: [{ emit: "event", level: "query" }],
});

chrome.setDefaultService(new chrome.ServiceBuilder(chromedriver.path).build());

bot.command("bubble", async (ctx) => {
  const chatId = ctx.chat?.id;

  if (!chatId) {
    return;
  }

  let stickerPack = await prisma.stickerPack.findUnique({ where: { chatId: chatId } });

  if (!stickerPack) {
    const sender = ctx.from?.id;

    if (!sender) {
      console.log("No sender id is provided");
      return;
    }

    const chat = ctx.chat;
    if (!chat) {
      console.log("No chat is provided somehow");
      return;
    }

    const chatPhotoId = await ctx.getChat().then((chat) => chat.photo?.small_file_id);
    if (!chatPhotoId) {
      console.log("No chat photo is provided");
      return;
    }

    const chatPhotoDownloadLink = await ctx.telegram.getFileLink(chatPhotoId);
    if (!chatPhotoDownloadLink) {
      console.log("No chat photo download path is provided");
      return;
    }

    let title: string;

    switch (chat.type) {
      case "group":
      case "channel":
      case "supergroup":
        title = chat.title;
        break;
      case "private": {
        const lastName = chat.last_name;
        title = `${chat.first_name}${lastName ? ` ${lastName}` : ""}`;
      }
    }
    title = `${title} bubbles`;

    const stickerPackName = `${chatId.toString().replace("-", "minus")}_by_${botName}`;
    console.log(
      `Creating sticker pack "${title}" (https://t.me/addstickers/${stickerPackName}) with ${ctx.from?.username} owner`
    );

    await ctx
      .createNewStickerSet(stickerPackName, title, {
        png_sticker: { source: "assets/logo-512.png" },
        emojis: "💭",
      })
      .catch((error) => {
        console.error(error);
        ctx.reply("Failed to create a new sticker pack");
      });

    const newStickerPack: StickerPack = {
      chatId: BigInt(chatId),
      userId: BigInt(sender),
      name: stickerPackName,
    };

    await prisma.stickerPack.create({ data: newStickerPack });
    stickerPack = newStickerPack;

    const photoBuffer = await got
      .stream(chatPhotoDownloadLink)
      .pipe(sharp({ failOnError: false }))
      .resize(100, 100)
      .toBuffer();

    const settingThumbResult = await ctx.setStickerSetThumb(stickerPackName, sender, { source: photoBuffer });
    console.log(`Setting sticker pack thumb: ${settingThumbResult}`);
  }

  const element = await new Builder()
    .forBrowser("chrome")
    .build()
    .then((driver) => {
      driver.get(resolve("./src/components.html"));
      return driver;
    })
    .then((it) => it.findElement(By.className("message")))
    .catch(console.error);

  if (!(element instanceof WebElement)) {
    return;
  }

  // TODO change element

  element
    .takeScreenshot()
    .then((img) => writeFile("./assets/screenshot.png", img, { flag: "w", encoding: "base64" }, console.error))
    .catch(console.error);

  ctx.replyWithSticker((await ctx.getStickerSet(stickerPack.name)).stickers[0].file_id);
});

bot.settings(async (ctx) => {
  const commands: readonly BotCommand[] = [
    {
      command: "/bubble",
      description: "create a bubble sticker",
    },
    {
      command: "/help",
      description: "display this message",
    },
  ];

  const commandList = commands.map((command) => `${command.command} - ${command.description}`).join("\n");
  console.log(`Setting commands:\n${commandList}`);

  const result = await ctx.setMyCommands?.(commands);
  console.log(`Commands set: ${result}`);

  ctx.reply("Ok");
});

bot.help(async (ctx) => {
  const commands = await ctx.getMyCommands();
  const info = commands.map((command) => `/${command.command} - ${command.description}`).join("\n");
  return ctx.reply(`Available commands:\n${info}`);
});

bot.use(async (ctx, next) => {
  const updateString = `Processing ${ctx.updateType} (${ctx.update.update_id}) in ${ctx.chat?.id}`;
  console.time(updateString);
  await next();
  console.timeEnd(updateString);
});

bot
  .launch()
  .then(() => console.log("Bot launched"))
  .catch(console.error)
  .finally(async () => await prisma.$disconnect());

process.once("SIGINT", () => bot.stop("SIGINT"));
process.once("SIGTERM", () => bot.stop("SIGTERM"));
