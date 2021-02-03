# Bubblifier Bot

[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square&logo=prettier)](https://github.com/prettier/prettier)
[![CodeFactor Grade](https://img.shields.io/codefactor/grade/github/pool-party/bubblifier-bot?logo=codefactor)](https://www.codefactor.io/repository/github/pool-party/bubblifier-bot)
[![TypeScript Lang](https://img.shields.io/github/languages/top/pool-party/bubblifier-bot?logo=TypeScript)](https://typescriptlang.org/)
[![Bubblifier Bot](https://img.shields.io/badge/telegram-Bubblifier_Bot-blue?logo=Telegram)](https://t.me/BubblifierBot/)

## Usage guide

### Building up a database

+ Install [Prisma](https://prisma.io/)
+ Run `prisma generate` to compile a custom `@prisma/client` to be used in code
+ Provide a database url in `DATABASE_URL` environment variable
+ Run `prisma migrate dev --preview-feature` to migrate your database

## Tools and Libraries

+ [Prisma](https://prisma.io/) - Next-generaion ORM for Node.js and TypeScript
+ [Telegraf](https://telegraf.js.org/) - Modern Telegram Bot Framework for Node.js

## Launching

+ Provide `BOT_TOKEN` - the one you get from [BotFather](https://t.me/BotFather)
