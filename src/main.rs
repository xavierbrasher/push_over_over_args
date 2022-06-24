use std::env::{args, Args, vars};
use dotenv::dotenv;
use reqwest;

struct Notification {
    token: String,
    user: String,
    message: String,
    title: String,
    responce: Option<reqwest::Response>
}


fn collect_args() -> String {
    let args: Args = args();
    let mut count: u16 = 0;
    let mut message: String = String::new();
    for argument in args {
        match count {
            0 => {count += 1; continue},
            1 => message = format!("{}", argument),
            _ => message = format!("{} {}", message, argument),
        }
        count += 1;
    }
    message
}

fn get_token_and_user() -> (String, String) {
    dotenv().ok();
    let mut user: String = String::new();
    let mut token: String = String::new();

    for (key, value) in vars() {
        if key == "USER" {
            user = value;
        }
        else if key == "TOKEN" {
            token = value;
        }
    }
    (user, token)
}

#[tokio::main]
async fn send_message(notification: &mut Notification) -> Result<(), Box<dyn std::error::Error>> {
    let params = [
        ("token", notification.token.clone()),
        ("user", notification.user.clone()),
        ("message", notification.message.clone()),
        ("title", notification.title.clone())
    ];
    let url = reqwest::Url::parse_with_params("https://api.pushover.net/1/messages.json", params);
    let client = reqwest::Client::new();
    let _resp = client.post(url.expect("wow")).send().await?;
    notification.responce = Some(_resp);
    Ok(())
}

fn main() {
    let message = collect_args();
    let (user, token) : (String, String) = get_token_and_user();
    let mut final_message: Notification = Notification { token: token, user: user, message: message, title: String::from("Rust Pushover"), responce: None };
    match send_message(&mut final_message) {
        Ok(_) => println!("Sent notification"),
        Err(e) => println!("Failed to send notification. Error: {:?}", e)
    }
}
