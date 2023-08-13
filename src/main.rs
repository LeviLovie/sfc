pub mod logger;
pub mod window;

use logger::*;
use window::*;

fn main() {
    log(LEVEL_DEBUG, "\x1b[1mSci-Fi Console emulator\x1b[0m has been started");
    let mut window = Window::new(1, "Sci-Fi Console".to_string());
    window.init();
}
