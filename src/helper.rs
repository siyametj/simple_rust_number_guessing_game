// random_number_guessing/src/helper.rs

use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        // if OS is windowds
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        // is OS is Linux/Mac
        Command::new("clear").status().unwrap();
    }
}
