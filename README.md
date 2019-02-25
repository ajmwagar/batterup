# batterup
🔋A small tool to remind you when you should unplug and plug in your laptop. Written in Rust.

__Note__: Right now this tool has only been tested on Linux, but I will probably work on OSX. No Windows support for the time being.

## Features
- Low Power consumption. Sleeps for 1 minute and then checks a file.
- Works over DBUS for notifications so no libnotify needed.
- Simple to setup.

## Installation

1. `git clone https://github.com/ajmwagar/batterup`
2. `cd batterup`
3. `cargo build --release && cargo install --path ./ --force`
4. run `batterup`
5. Profit! And enjoy long battery health.
