# notifications

Send notifications from the CLI. Desktop toasts and Telegram.

## Usage

```
notifications "message"
notifications "title" "message"
notifications "title" "message" "https://example.com"
notifications "title" "message" "C:\path\to\folder"
```

### Flags

```
(none)    Desktop toast only (default)
--tg      Telegram only
--all     Desktop toast + Telegram
```

Clicking a desktop notification with a link opens it in the default handler (browser, Explorer, or associated app).

## Telegram Setup

1. Create a bot via [@BotFather](https://t.me/BotFather)
2. `/start` the bot in Telegram
3. Place `notifications.json` next to the exe:

```json
{
  "bot_token": "123456:ABC-xyz",
  "chat_id": 123456789
}
```

`chat_id` is your Telegram user ID.

## Build

```
build.bat
```

## License

MIT
