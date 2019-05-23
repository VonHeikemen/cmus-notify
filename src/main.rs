use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::process::{Child, Command};

struct Notification<'a> {
    title: &'a str,
    body: String,
}

impl<'a> Notification<'a> {
    fn new(args: &HashMap<String, String>) -> Notification {
        fn not_empty(val: &&String) -> bool {
            !val.is_empty()
        }

        let file = || {
            args.get("file")
                .filter(not_empty)
                .map(|str| match str.rfind('/') {
                    Some(val) => &str.split_at(val).1[1..],
                    None => str.as_str(),
                })
        };

        let title = args.get("title").filter(not_empty).map(|str| str.as_str());

        let artist = args
            .get("artist")
            .filter(not_empty)
            .or(Some(&String::from("Unknown")))
            .map(|str| format!("<b>Artist:</b> {}", str));

        let mut message = artist.unwrap();

        args.get("album")
            .filter(not_empty)
            .map(|str| format!("\n<b>Album:</b> {}", str))
            .map(|str| message.push_str(&str));

        Notification {
            title: title.or_else(file).unwrap_or("Unknown song"),
            body: message,
        }
    }
}

fn parse_args(args: &Vec<String>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let range = 3..args.len();
    let empty = String::from("");

    for index in range.step_by(2) {
        let arg_val = args.get(index + 1).unwrap_or(&empty).to_string();
        result.insert(args[index].to_string(), arg_val);
    }

    result
}

fn show_notification(args: &Vec<String>) -> Result<Child, Error> {
    let arg_list = parse_args(&args);
    let notification = Notification::new(&arg_list);
    Command::new("notify-send")
        .arg(notification.title)
        .arg(notification.body)
        .spawn()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    args.get(1..3)
        .filter(|val| val == &["status", "playing"])
        .and_then(|_| show_notification(&args).ok());
}
