use std::env;
use std::fs;

fn getConfigDirPath() -> String {
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Unable to get home directory.");
            return Ok(());
        }
    };
}