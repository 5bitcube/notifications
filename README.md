# notifications

Windows toast notifications from the CLI. 170KB Rust binary.

## Usage

```
notifications "message"
notifications "title" "message"
notifications "title" "message" "https://example.com"
notifications "title" "message" "C:\path\to\folder"
notifications "title" "message" "C:\path\to\file.txt"
```

Clicking a notification with a link opens it in the default handler (browser, Explorer, or associated app).

## Build

```
build.bat
```

## License

MIT
