extern crate image;
extern crate num;
extern crate num_traits;
extern crate gtk;
// import the macro of this crate
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::Widget;

mod gui;

fn main() {
    gui::Win::run(()).unwrap();

}
