mod utils;

#[cfg(feature = "squares")]
mod app {
    mod squares;

    pub use squares::*;
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    nannou::app(app::model)
        .update(app::update)
        .size(800, 800)
        .simple_window(app::view)
        .run();
}
