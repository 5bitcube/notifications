use std::env;
use std::process;
use win_toast_notify::WinToastNotify;

// App ID for Windows toast notifications.
// Uses PowerShell's full registered AUMID so Windows 11 actually displays toasts.
const APP_ID: &str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: notifications.exe \"message\"");
        eprintln!("       notifications.exe \"title\" \"message\"");
        eprintln!("       notifications.exe \"title\" \"message\" \"https://...\"");
        process::exit(1);
    }

    // Parse args: 1 = message only, 2 = title + message, 3 = title + message + link
    let (title, body, link) = match args.len() {
        2 => ("Notification", args[1].as_str(), None),
        3 => (args[1].as_str(), args[2].as_str(), None),
        _ => (args[1].as_str(), args[2].as_str(), Some(args[3].as_str())),
    };

    let mut toast = WinToastNotify::new()
        .set_app_id(APP_ID)
        .set_title(title)
        .set_messages(vec![body]);

    // Clicking the notification opens the link in the default browser
    if let Some(url) = link {
        toast = toast.set_open(url);
    }

    if let Err(e) = toast.show() {
        eprintln!("Failed to show notification: {}", e);
        process::exit(1);
    }
}
