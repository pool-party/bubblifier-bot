# Bubblifier Bot

[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square&logo=prettier)](https://github.com/prettier/prettier)
[![CodeFactor Grade](https://img.shields.io/codefactor/grade/github/pool-party/bubblifier-bot?logo=codefactor)](https://www.codefactor.io/repository/github/pool-party/bubblifier-bot)
[![TypeScript Lang](https://img.shields.io/github/languages/top/pool-party/bubblifier-bot?logo=TypeScript)](http://typescriptlang.org/)
[![Bubblifier Bot](https://img.shields.io/badge/telegram-Bubblifier_Bot-blue?logo=Telegram)](https://t.me/BubblifierBot/)

## Usage guide

Add [`@BubblifierBot`](https://t.me/BubblifierBot) to the chat to be able to create custom bubble sticker packs using
following commands:

+ `/bubble` - on first invocation it both creates a new sticker pack for the chat
  and adds referenced message bubble sticker, on the following invocations it only adds new bubble stickers to the
  existing pack

## Tools and Libraries

+ [TypeScript](http://typescriptlang.org/) language

+ [npm](https://npmjs.com/) - JavaScript programming language package manager

+ [Prettier](https://prettier.io/) - An opinionated code formatter

+ [ESLint](https://eslint.org/) - Static code analyzer

+ [Emoji commit messages guide](https://gitmoji.dev/)

+ [Prisma](https://prisma.io/) - Next-generaion ORM for Node.js and TypeScript

+ [Telegraf](https://telegraf.js.org/) - Modern Telegram Bot Framework for Node.js

+ [sharp](https://github.com/lovell/sharp/) - High performance Node.js image processing,
  the fastest module to resize JPEG, PNG, WebP, AVIF and TIFF images

+ [got](https://github.com/sindresorhus/got/) - Human-friendly and powerful HTTP request library for Node.js

## Launching

### Building up a database

+ Install [Prisma](https://prisma.io/)

+ Run `prisma generate` to compile a custom `@prisma/client` to be used in code

+ Provide a database url in `DATABASE_URL` environment variable

+ Run `prisma migrate dev --preview-feature` to migrate your database

### Environment variables

+ Provide `DATABASE_URL` with database url you want to use

+ Provide `BOT_TOKEN` - the one you get from [BotFather](https://t.me/BotFather)

+ Provide `BOT_NAME` - the registered bot name
