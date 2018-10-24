extern crate modscript;
extern crate serde_json;
extern crate pancurses;

mod lib;
mod textrender;
mod global;
mod entity;
mod level;
mod layout;

use std::rc::Rc;
use std::cell::RefCell;

pub type Coord = (usize, usize);

fn main() {
    // init graphics, loading screen

    let glob = Rc::new(RefCell::new(global::Global::new()));

    // init libraries
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap().attach_package(lib::math::NAME, lib::math::call_ref());


    // TODO: get from arg
    let hub_file = "example/rogue.hub.json";

    glob.borrow_mut().init_game(hub_file).unwrap();

    // run
    run();
}

fn run() {
    // call init script

    // loop:
        // wait for input

        // call script depending on input

        // call render script

        // render output

    // exit
}
