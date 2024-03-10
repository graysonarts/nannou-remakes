#![allow(dead_code)]
#![allow(unused)]
use app::{HEIGHT, WIDTH};
use nannou::{app::ModelFn, App};

mod utils;

macro_rules! remake {
    ($id:ident, $feat:literal) => {
        #[cfg(feature = $feat)]
        mod app {
            mod $id;

            pub use $id::*;
        }
    };
}

macro_rules! map_range {
    ($value:expr, $from_min:expr, $from_max:expr, $to_min:expr, $to_max:expr) => {
        if $from_min == $from_max {
            ($to_max + $to_min) / 2
        } else {
            map_range($value, $from_min, $from_max, $to_min, $to_max)
        }
    };
}

remake!(concord, "concord");
remake!(squares, "squares");
remake!(tangle, "tangle");
remake!(playground, "playground");

pub fn height() -> u32 {
    #[cfg(feature = "lazy_height")]
    {
        *HEIGHT
    }
    #[cfg(not(feature = "lazy_height"))]
    {
        HEIGHT
    }
}

fn model_wrapper(app: &App) -> app::Model {
    #[cfg(feature = "once")]
    {
        app.set_loop_mode(nannou::app::LoopMode::NTimes {
            number_of_updates: 1,
        });
    }
    app::model(app)
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    #[cfg(feature = "simple")]
    {
        nannou::app(model_wrapper)
            .update(app::update)
            .size(WIDTH, height())
            .simple_window(app::view)
            .run();
    }
    #[cfg(not(feature = "simple"))]
    {
        nannou::app(model_wrapper).update(app::update).run();
    }
}
