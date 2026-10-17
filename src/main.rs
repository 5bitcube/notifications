use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use serde::Deserialize;
use win_toast_notify::WinToastNotify;

// App ID for Windows toast notifications.
// Uses PowerShell's full registered AUMID so Windows 11 actually displays toasts.
const APP_ID: &str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

#[derive(Deserialize)]
struct Config {
    bot_token: String,
    chat_id: i64,
}

fn usage() {
    eprintln!("Usage: notifications [--tg|--all] \"message\"");
    eprintln!("       notifications [--tg|--all] \"title\" \"message\"");
    eprintln!("       notifications [--tg|--all] \"title\" \"message\" \"https://...\"");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  (none)  Desktop toast only (default)");
    eprintln!("  --tg    Telegram only");
    eprintln!("  --all   Desktop toast + Telegram");
    eprintln!();
    eprintln!("Telegram requires notifications.json next to the exe:");
    eprintln!("  {{ \"bot_token\": \"123:ABC\", \"chat_id\": 123456789 }}");
}

/// Locate config file next to the executable
fn config_path() -> PathBuf {
    let exe = env::current_exe().expect("cannot resolve exe path");
    exe.parent().unwrap().join("notifications.json")
}

/// Load Telegram bot config from telegram.json
fn load_config() -> Result<Config, String> {
    let path = config_path();
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
    serde_json::from_str(&data)
        .map_err(|e| format!("Invalid {}: {}", path.display(), e))
}

/// Send a desktop toast notification
fn send_desktop(title: &str, body: &str, link: Option<&str>) {
    let mut toast = WinToastNotify::new()
        .set_app_id(APP_ID)
        .set_title(title)
        .set_messages(vec![body]);

    if let Some(url) = link {
        toast = toast.set_open(url);
    }

    if let Err(e) = toast.show() {
        eprintln!("Desktop notification failed: {}", e);
        process::exit(1);
    }
}

/// Send a Telegram message via Bot API
fn send_telegram(config: &Config, title: &str, body: &str, link: Option<&str>) {
    // Format the message with optional link
    let text = match link {
        Some(url) => format!("<b>{}</b>\n{}\n\n<a href=\"{}\">{}</a>", title, body, url, url),
        None => format!("<b>{}</b>\n{}", title, body),
    };

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config.bot_token
    );

    let response = ureq::post(&url)
        .send_json(serde_json::json!({
            "chat_id": config.chat_id,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": false
        }));

    match response {
        Ok(resp) => {
            if resp.status() != 200 {
                eprintln!("Telegram API returned status {}", resp.status());
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Telegram send failed: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    // Separate flags from positional args
    let mut send_desktop_flag = true;
    let mut send_telegram_flag = false;
    let mut positional: Vec<&str> = Vec::new();

    for arg in &raw_args[1..] {
        match arg.as_str() {
            "--tg" => {
                send_desktop_flag = false;
                send_telegram_flag = true;
            }
            "--all" => {
                send_desktop_flag = true;
                send_telegram_flag = true;
            }
            "--help" | "-h" => {
                usage();
                return;
            }
            _ => positional.push(arg),
        }
    }

    if positional.is_empty() {
        usage();
        process::exit(1);
    }

    // Parse positional args: 1 = message, 2 = title + message, 3 = title + message + link
    let (title, body, link) = match positional.len() {
        1 => ("Notification", positional[0], None),
        2 => (positional[0], positional[1], None),
        _ => (positional[0], positional[1], Some(positional[2])),
    };

    if send_desktop_flag {
        send_desktop(title, body, link);
    }

    if send_telegram_flag {
        let config = match load_config() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        send_telegram(&config, title, body, link);
    }
}
