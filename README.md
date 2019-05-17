## Requirements
- [Rust](https://github.com/rust-lang/rust)
- [notify-send](https://ss64.com/bash/notify-send.html)

## Instalation
1. Download or clone this repository and navigate to the project folder.
2. Compile using `cargo build --release`
3. Set the status_display_program variable in cmus:
> :set status_display_program=/project-folder/target/release/cmus-notify
4. Save the changes using:
> :save

## Other solutions
- [status_diplay_notify_send.py](https://github.com/cmus/cmus/wiki/status_diplay_notify_send.py)
- [CmusNotify (go)](https://github.com/sebojanko/CmusNotify)
- [cmus_notify (bash)](https://gist.github.com/VonHeikemen/1eb9c0f9edef923100fbab41a65049a2)
