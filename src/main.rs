#![allow(unused_imports)]

extern crate image;
extern crate num;
extern crate num_traits;
extern crate gtk;
// import the macro of this crate
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use num::NumCast;
use num_traits::clamp;
extern crate time;


use relm::Widget;

mod cayon;
mod gui;

fn main() {
    gui::Win::run(()).unwrap();
}
