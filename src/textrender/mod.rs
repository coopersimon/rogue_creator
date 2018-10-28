// For rendering stuff to screen
// Takes in configuration (generated by scripts & engine) and prints output
// Doesn't communicate with scripting engine at all!
// Output will be text to start, might add tile based 2d later

mod textbox;
mod map;
mod printbox;

use Coord;
use self::map::Map;
use self::printbox::PrintBox;

pub use self::map::MapCommand;
pub use self::printbox::PrintCommand;

use pancurses::Window;

use std::sync::mpsc::Receiver;

// Coords: 0,0 is top left.

pub trait Render {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord);
}

pub enum RenderCommand {
    Renderable(Box<Render>, Coord, Coord),
    Map(Coord, Coord),
    PrintBox(Coord, Coord),
}

pub struct RenderData {
    lib_recv: Receiver<RenderCommand>,
    map: Map,
    printbox: PrintBox,
}

impl RenderData {
    // Todo: make channels inside constructor?
    pub fn new(recv: Receiver<RenderCommand>, map_recv: Receiver<MapCommand>, printbox_recv: Receiver<PrintCommand>) -> Self {
        RenderData {
            lib_recv: recv,
            map: Map::new(map_recv),
            printbox: PrintBox::new(printbox_recv),
        }
    }

    pub fn render(&mut self, w: &mut Window) {
        use self::RenderCommand::*;
        w.clear();

        let mut iter = self.lib_recv.iter();
        while let Some(c) = iter.next() {
            match c {
                Renderable(mut r, tl, br)   => r.render(w, tl, br),
                Map(tl, br)                 => self.map.render(w, tl, br),
                PrintBox(tl, br)            => self.printbox.render(w, tl, br),
            }
        }

        w.refresh();
    }
}
