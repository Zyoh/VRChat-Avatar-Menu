# VRChat-Avatar-Menu
 
External menu for VRChat avatars.

# Usage

The program will ask for an IP address, port, and an avatar config file.

The `address:port` should be where your game client is listening. This defaults to VRChat's default if you leave it empty which is `127.0.0.1:9000`.

To find your avatar config:

- Go to `%UserProfile%/AppData/LocalLow/VRChat/VRChat/OSC`
    - Paste into explorer address bar or go there manually

- Open the folder with your user ID
    - There's probably only one unless you logged into multiple accounts.

- Open the `Avatars` folder and choose the file with your avatar's ID
    - One way to find your avatar's ID:
        - Go to https://vrchat.com/home/avatars
        - Click your avatar's thumbnail
        - Look at the `avtr_...` string in the url

# Installation

## [Build it yourself using these build instructions](#build)

or, if you're ok with running exe files from the internet, download one from 
[here](https://github.com/Zyoh/VRChat-Avatar-Menu/releases/latest/download/vrchat-avatar-menu.exe)

# Build

Install Rust/cargo

Run `cargo b --release`

Find the exe file in the `./target/release/` directory
