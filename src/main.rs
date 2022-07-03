use std::env::{args, vars};
use dotenv::dotenv;
use reqwest::{self, Response};

pub fn input() -> String {
    // gets stdin and sets to buffer
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    // get tmps and removes the last 2 characters of it    
    let mut tmp: String = String::new();
    let mut tmp_chars = input.chars();
    for x in 0..input.len() {
        if cfg!(unix) {
            if x == (input.len() - 1) {
                break;
            }
        }
        else if cfg!(windows) {
            if x == (input.len() - 2) {
                break;
            }
        }
        
        tmp.push(tmp_chars.nth(0).unwrap());
    }
    // returns the modified tmp
    return tmp;
}

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
TOKEN                   The application token in pushover (necessary)
TITLE                   The Notifications defualt title, Defult is useally \"Rust Push Notification\" (optional)
";

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

fn get_token_and_user(notification: &mut Notification) -> bool {
    dotenv().ok();
    let mut user: String = String::new();
    let mut token: String = String::new();
    let mut title: String = String::new();
    for (key, value) in vars() {
        if key == "USERAPI" {
            user = value;
        }
        else if key == "TOKEN" {
            token = value;
        }
        else if key == "TITLE" {
            title = value;
        }
    }
    if user == "" || token == "" {
        return false;
    }
    notification.user = user;
    notification.token = token;
    if notification.title == "Rust Push Message" && title != "" {
        notification.title = title
    }
    true
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
    let worked: bool = get_token_and_user(&mut message);
    if worked == false {
        return println!("No USERAPI or TOKEN enviromental variables");
    }

    match send_message(&mut message) {
        Ok(_) => println!("Sent notification"),
        Err(e) => println!("Failed to send notification. Error: {:#?}", e)
    }
}
