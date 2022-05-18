# rustparty

A networked, CLI version of the online game Bombparty written entirely in Rust. Enables up to 8 players to attempt to match words to a prompt within a timer until somebody runs out of lives.

The original game is located [here](https://jklm.fun).
## Installation

> rustparty is not tested on Windows. If not functional, try WSL. Installation can be found [here](https://docs.microsoft.com/en-us/windows/wsl/install).

To install, just download the binary and add it to your path. The binary is located both in the release. For example:

`mkdir ~/.rustparty/`

`curl https://github.com/erakai/rustparty/releases/download/1.0/rustparty >> ~/.rustparty/rustparty`

`chmod +x ~/.rustparty/rustparty`

`export PATH=~/.rustparty:$PATH`

Alternatively, download rust, clone this repo, and run `cargo build --release` to generate the binary manually.

### 

## Contributors
- Kai Tinkess

## Screenshots

![2players](screenshots/2players.png)
![5players](screenshots/5players.png)
![server](screenshots/server.png)
