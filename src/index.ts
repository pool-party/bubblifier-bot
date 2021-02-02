import { Telegraf } from "telegraf";
import { BotCommand } from "telegraf/src/telegram-types";

const token = process.env.BOT_TOKEN;
if (token === undefined) {
  throw new Error("BOT_TOKEN must be provided");
}

const bot = new Telegraf(token);

bot.use(async (ctx, next) => {
  const updateString = `Processing ${ctx.updateType} (${ctx.update.update_id}) in ${ctx.chat?.id}`;
  console.time(updateString);
  await next();
  console.timeEnd(updateString);
});

bot.command("bubble", async (ctx) => {
  // TODO
  // const chat = await ctx.getChat();
  // await ctx.createNewStickerSet(chat.id.toString(), chat.id.toString());
});

// TODO
bot.settings(async (ctx) => {
  const commands: readonly BotCommand[] = [
    {
      command: "/bubble",
      description: "some description",
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

bot
  .launch()
  .then(() => console.log("Bot launched"))
  .catch((err) => console.log(err));

process.once("SIGINT", () => bot.stop("SIGINT"));
process.once("SIGTERM", () => bot.stop("SIGTERM"));
