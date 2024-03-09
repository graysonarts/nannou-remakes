#![allow(dead_code)]

use app::{HEIGHT, WIDTH};

mod utils;

#[cfg(feature = "squares")]
mod app {
    mod squares;

    pub use squares::*;
}

#[cfg(feature = "tangle")]
mod app {
    mod tangle;

    pub use tangle::*;
}

#[cfg(feature = "playground")]
mod app {
    mod playground;

    pub use playground::*;
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    nannou::app(app::model)
        .update(app::update)
        .size(WIDTH, HEIGHT)
        .simple_window(app::view)
        .run();
}
