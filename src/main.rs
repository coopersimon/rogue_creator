extern crate modscript;
extern crate serde_json;
extern crate pancurses;
extern crate rand;

mod lib;
mod textrender;
mod global;
mod entity;
mod level;
mod layout;
//mod state;

use pancurses::{initscr, endwin, set_title, noecho, Window, Input};

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::channel;

pub type Coord = (usize, usize);

fn main() {
    // init graphics, loading screen

    // Channels and objects
    let (s_rend, r_rend) = channel();

    let glob = Rc::new(RefCell::new(global::Global::new()));
    //let state = Rc::new(RefCell::new(state::State::new()));

    // init libraries
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::math::NAME, lib::math::call_ref());
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::txtrend::NAME, lib::txtrend::call_ref(s_rend));
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::glob::NAME, lib::glob::call_ref(glob.clone()));
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::level::NAME, lib::level::call_ref(glob.clone()));
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::entity::NAME, lib::entity::call_ref(glob.clone()));

    // TODO: get from arg
    let hub_file = "example/rogue.hub.json";

    glob.borrow_mut().init_game(hub_file).unwrap();

    // TODO: get from hub file
    let window = init_terminal("Rogue");

    //Rc::get_mut(&mut state.borrow_mut().glob_obj) = glob.init.call();

    // run
    loop {
        //renderer.render(&mut window);
        match window.getch() {
            Some(Input::Character(c)) => {glob.borrow().run_input(c).unwrap();},
            Some(_) => (), // TODO: special char support
            None => (),
        }
    }

    endwin();
}

fn init_terminal(name: &str) -> Window {
    let window = initscr();
    window.keypad(true);
    //resize_term();
    set_title(name);
    noecho();
    window
}
