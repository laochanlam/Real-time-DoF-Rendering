#![allow(unused_imports)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate image;
extern crate num;
extern crate num_traits;
extern crate gtk;

use relm::Widget;

mod cayon;
mod gui;

fn main() {
    gui::Win::run(()).unwrap();
}
