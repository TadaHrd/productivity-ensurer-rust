#![windows_subsystem = "windows"]

use std::{thread::sleep, time::Duration};

fn main() -> ! {
    loop {
        if msgbox::create(
            "Are you being productive?",
            "If yes, good!\nIf no, get to work loser.",
            msgbox::IconType::Error,
        )
        .is_err()
        {
            panic!("error: couldn't create popup");
        }
        sleep(Duration::from_secs(300))
    }
}
