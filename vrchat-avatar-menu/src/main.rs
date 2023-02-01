// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

#![windows_subsystem = "windows"]

mod ui;
mod params;

use vrchat_osc::VRChatOSC;

use std::error::Error;

lazy_static::lazy_static! {
    static ref ENGINE: VRChatOSC = VRChatOSC {
        ..Default::default()
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    ui::launch();

    Ok(())
}
