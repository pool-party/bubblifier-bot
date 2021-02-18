# Bubblifier Bot

[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg?style=flat-square&logo=prettier)](https://github.com/prettier/prettier)
[![CodeFactor Grade](https://img.shields.io/codefactor/grade/github/pool-party/bubblifier-bot?logo=codefactor)](https://www.codefactor.io/repository/github/pool-party/bubblifier-bot)
[![Bubblifier Bot](https://img.shields.io/badge/telegram-Bubblifier_Bot-blue?logo=Telegram)](https://t.me/BubblifierBot/)

<div align="center">
    <img src="assets/title.png" width=50% alt="logo">
</div>
<p align="center">© 2020 Pool Party Corp. All rights reserved.</p>

## Usage guide

Add [`@BubblifierBot`](https://t.me/BubblifierBot) to the chat to be able to create custom bubble sticker packs using
following commands:

- `/bubble` - on first invocation it both creates a new sticker pack for the chat
  and adds referenced message bubble sticker, on the following invocations it only adds new bubble stickers to the
  existing pack

## Structure

![Project Structure](assets/project-schema.jpg)

## Tools and Libraries

### Bubble rendering server ([client/](client/))

- [TypeScript](http://typescriptlang.org/) language

- [React](https://reactjs.org/) - a JavaScript library for building user interfaces
