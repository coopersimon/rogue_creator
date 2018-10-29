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
mod textitem;
mod tile;

use pancurses::{initscr, endwin, set_title, noecho, Window, Input};

use textrender::RenderData;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::channel;

pub type Coord = (usize, usize);

fn main() {
    // init graphics, loading screen

    // Channels and objects
    let (s_rend, r_rend) = channel();
    let (s_pbox, r_pbox) = channel();
    let (s_map, r_map) = channel();

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
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::pbox::NAME, lib::pbox::call_ref(s_pbox));
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::map::NAME, lib::map::call_ref(s_map));
    Rc::get_mut(&mut glob.borrow_mut().source).unwrap()
        .attach_package(lib::makemap::NAME, lib::makemap::call_ref(glob.clone()));


    // TODO: get from arg
    let hub_file = "example/rogue.hub.json";

    glob.borrow_mut().init_game(hub_file).unwrap();

    // TODO: get from hub file
    let mut window = init_terminal("Rogue");

    let mut renderer = RenderData::new(r_rend, r_map, r_pbox);

    // run
    loop {
        renderer.render(&mut window);
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
