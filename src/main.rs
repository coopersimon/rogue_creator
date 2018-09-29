extern crate modscript;
extern crate serde_json;

//mod lib;
//mod render;
mod global;
mod entity;
mod level;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub type Coord = (usize, usize);

fn main() {
    // init graphics, loading screen

    let mut glob = Rc::new(RefCell::new(global::Global::new()));

    // let libs = glob.borrow_mut().source.get_mut().unwrap();
    
    // init libraries

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
