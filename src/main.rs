pub mod logger;
use logger::*;

fn main() {
    log(LEVEL_DEBUG, "\x1b[1mSci-Fi Console emulator\x1b[0m has been started");
}
