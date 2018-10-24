use super::Render;
use std::sync::mpsc::Receiver;
use Coord;
use pancurses::Window;

pub enum PrintCommand {
    TopLeft(Coord),
    BottomRight(Coord),
    NewText(String),
    Next,
    Clear,
}

pub struct PrintBox {
    top_left: Coord,
    bottom_right: Coord,
    display_text: Vec<String>,
    lib_recv: Receiver<PrintCommand>,
}

impl PrintBox {
    pub fn new(tl: Coord, br: Coord, recv: Receiver<PrintCommand>) -> Self {
        PrintBox {
            top_left: tl,
            bottom_right: br,
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
                TopLeft(v)              => self.top_left = v,
                BottomRight(v)          => self.bottom_right = v,
                Next                    => {self.display_text.pop();},
                Clear                   => self.display_text.clear(),
            }
        }
    }
}

impl Render for PrintBox {
    fn render(&mut self, w: &mut Window) {
        self.process_commands();

        let length = self.bottom_right.0 - self.top_left.0;
        if self.display_text.len() > 0 {
            w.mvaddnstr(self.top_left.1 as i32, self.top_left.0 as i32, &self.display_text[0], length as i32);
        }
    }
}
