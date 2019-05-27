use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::process::{Child, Command};

struct Notification {
    title: String,
    body: String,
}

impl Notification {
    fn new(args: HashMap<String, String>) -> Notification {
        fn not_empty(val: &&String) -> bool {
            !val.is_empty()
        }

        let file = || {
            args.get("file")
                .filter(not_empty)
                .map(|str| match str.rfind('/') {
                    Some(position) => str.split_at(position + 1).1,
                    None => str.as_str(),
                })
        };

        let title = args.get("title").filter(not_empty).map(|str| str.as_str());

        let artist = args
            .get("artist")
            .filter(not_empty)
            .or(Some(&String::from("Unknown")))
            .map(|str| format!("<b>Artist:</b> {}", str));

        let album = args
            .get("album")
            .filter(not_empty)
            .map(|str| format!("<b>Album:</b> {}", str));

        let message = album
            .iter()
            .fold(artist.unwrap(), |acc, val| format!("{}\n{}", acc, val));

        Notification {
            title: title.or_else(file).unwrap_or("Unknown song").to_string(),
            body: message,
        }
    }

    fn show(self) -> Result<Child, Error> {
        Command::new("notify-send")
            .arg(self.title)
            .arg(self.body)
            .spawn()
    }
}

fn parse_args(args: &Vec<String>) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let empty = String::from("");

    for chunk in args.chunks(2) {
        let val = chunk.get(1).unwrap_or(&empty);
        result.insert(chunk[0].to_string(), val.to_string());
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let run = |_| Notification::new(parse_args(&args)).show().ok();

    args.get(0..2)
        .filter(|val| val == &["status", "playing"])
        .and_then(run);
}
