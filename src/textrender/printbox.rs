use super::Render;
use Coord;
use pancurses::Window;

use std::sync::mpsc::Receiver;

pub enum PrintCommand {
    NewText(String),
    Next,
    Clear,
}

pub struct PrintBox {
    display_text: Vec<String>,
    lib_recv: Receiver<PrintCommand>,
}

impl PrintBox {
    pub fn new(recv: Receiver<PrintCommand>) -> Self {
        PrintBox {
            display_text: Vec::new(),
            lib_recv: recv,
        }
    }

    fn process_commands(&mut self) {
        use self::PrintCommand::*;

        let mut iter = self.lib_recv.iter();
        while let Some(c) = iter.next() {
            match c {
                NewText(v)              => self.display_text.push(v), // Todo: split and push
                Next                    => {self.display_text.pop();},
                Clear                   => self.display_text.clear(),
            }
        }
    }
}

impl Render for PrintBox {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord) {
        self.process_commands();

        // TODO: split into lines
        let length = bottom_right.0 - top_left.0;
        if self.display_text.len() > 0 {
            w.mvaddnstr(top_left.1 as i32, top_left.0 as i32, &self.display_text[0], length as i32);
        }
    }
}
