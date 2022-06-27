use std::env::{args, vars};
use dotenv::dotenv;
use reqwest::{self, Response};
mod default_libary;
use default_libary::*;

static HELPMESSAGE: &str = 
"Usage: push_over_over_args [options] [message]
       push_over_over_args [options] [message in quotes]

Option:
    -                   Reads arguments from stdin
    --                  Ignores any options after
    -t, --title         The title of the pushover message
    -u, --url           Includes a URL in the message

Environment variables:

USERAPI                 The user api key in pushover (necessary)
TOKEN                   The application token in pushover (necessary)";

struct Notification {
    token: String,
    user: String,
    message: String,
    title: String,
    url: String,
    responce: Option<Response>
}

fn collect_args() -> Notification {
    let args: Vec<String> = args().collect();
    let mut message: Notification = Notification { token: String::from(""), user: String::from(""), message: String::from(""), title: String::from("Rust Push Message"), url: String::from(""), responce: None };
    let mut skip_next_option: bool = false;
    let mut ignore_options: bool = false;
    if args.len() == 1 {
        message.message = String::from("NOARGS_092_124+43");
        return message;
    }
    for i in 0..args.len() {
        if i == 0 {continue;}
        if skip_next_option == true {skip_next_option = false; continue;}
        if args[i] == "--" {ignore_options = true; continue;}
        if ignore_options == false {
            if args[i] == "-t" || args[i] == "--title" {
                if args.len() -1 >= i + 1 {
                    message.title = args[i+1].clone();
                    skip_next_option = true;
                    continue;
                }
            }
            if args[i] == "-u" || args[i] == "--url" {
                if args.len() -1 >= i + 1 {
                    message.url = args[i+1].clone();
                    skip_next_option = true;
                    continue;
                }
            }
            if args[i] == "-" {
                message.message = format!("{}{} ", message.message,  input());
                break;
            }
        }
        message.message = format!("{}{} ", message.message, args[i])
    }
    message
}

fn get_token_and_user() -> (String, String) {
    dotenv().ok();
    let mut user: String = String::new();
    let mut token: String = String::new();

    for (key, value) in vars() {
        if key == "USERAPI" {
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
        ("title", notification.title.clone()),
        ("url", notification.url.clone())
    ];
    let url = reqwest::Url::parse_with_params("https://api.pushover.net/1/messages.json", params);
    let client = reqwest::Client::new();
    let _resp = client.post(url.expect("wow")).send().await?;
    notification.responce = Some(_resp);
    Ok(())
}

fn main() {
    let mut message: Notification = collect_args();
    if message.message == "NOARGS_092_124+43" {
        println!("{}", HELPMESSAGE);
        return;
    }
    let (user, token) : (String, String) = get_token_and_user();
    (message.user, message.token) = (user, token);
    match send_message(&mut message) {
        Ok(_) => println!("Sent notification"),
        Err(e) => println!("Failed to send notification. Error: {:#?}", e)
    }
}
