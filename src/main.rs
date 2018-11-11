extern crate modscript;
extern crate serde_json;
extern crate pancurses;
extern crate rand;

mod error;
mod lib;
mod textrender;
mod global;
mod entity;
mod level;
mod layout;
mod textitem;
mod tile;
mod state;

use pancurses::{initscr, endwin, set_title, noecho, Window, Input};

use textrender::RenderData;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::channel;
use std::{thread, time};

pub type Coord = (usize, usize);

pub enum MainCommand {
    EndGame,
    Terminate,
    Wait(u64),
}

fn main() {
    // init graphics, loading screen

    // Channels and objects
    let (s_rend, r_rend) = channel();
    let (s_pbox, r_pbox) = channel();
    let (s_map,  r_map)  = channel();
    let (s_main, r_main) = channel();

    let glob = Rc::new(RefCell::new(global::Global::new()));
    let state = Rc::new(RefCell::new(state::State::new(s_map.clone())));

    // init libraries
    glob.borrow_mut().source.attach_package(lib::math::NAME, lib::math::call_ref());
    glob.borrow_mut().source.attach_package(lib::txtrend::NAME, lib::txtrend::call_ref(s_rend));
    glob.borrow_mut().source.attach_package(lib::glob::NAME, lib::glob::call_ref(state.clone()));
    glob.borrow_mut().source.attach_package(lib::level::NAME, lib::level::call_ref(glob.clone(), state.clone()));
    glob.borrow_mut().source.attach_package(lib::entity::NAME, lib::entity::call_ref(glob.clone(), state.clone()));
    glob.borrow_mut().source.attach_package(lib::pbox::NAME, lib::pbox::call_ref(s_pbox));
    glob.borrow_mut().source.attach_package(lib::map::NAME, lib::map::call_ref(s_map));
    glob.borrow_mut().source.attach_package(lib::makemap::NAME, lib::makemap::call_ref(state.clone()));
    glob.borrow_mut().source.attach_package(lib::control::NAME, lib::control::call_ref(s_main));


    // TODO: get from arg
    let hub_file = "rogue/hub.json";

    glob.borrow_mut().init_game(hub_file).unwrap();
    let glob_obj = glob.borrow().init().unwrap();
    state.borrow_mut().set_glob_obj(glob_obj);

    // TODO: get from hub file
    let mut window = init_terminal("Rogue");

    let mut renderer = RenderData::new(r_rend, r_map, r_pbox);

    // run
    loop {
        // render
        state.borrow().prepare_render();
        glob.borrow().run_render(state.borrow().get_current_layout());
        renderer.render(&mut window);

        // check outputs
        let mut should_break = false;
        let mut iter = r_main.try_iter();
        while let Some(c) = iter.next() {
            match c {
                MainCommand::Wait(ms) => thread::sleep(time::Duration::from_millis(ms)),
                MainCommand::EndGame => {glob.borrow().end().unwrap();},
                MainCommand::Terminate => should_break = true,
            };
        }
        if should_break {
            break;
        }

        // input
        match window.getch() {
            Some(Input::Character(c)) => {glob.borrow().run_input(state.borrow().get_current_layout(), c).unwrap();},
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
