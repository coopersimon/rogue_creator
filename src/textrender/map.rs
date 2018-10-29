use super::Render;
use Coord;
use pancurses::Window;

use std::sync::mpsc::Receiver;

pub enum MapCommand {
    Display(Coord, Coord),
    MapData(Vec<Vec<char>>),
    SetDisplayArea(Coord, Coord, bool),
    ToggleDisplayArea(Coord, Coord),
}

pub struct Map {
    display_top_left: Coord,
    display_bottom_right: Coord,
    map_data: Vec<Vec<char>>,
    display_area: Vec<Vec<bool>>,
    lib_recv: Receiver<MapCommand>,
}

impl Map {
    pub fn new(recv: Receiver<MapCommand>) -> Self {
        Map {
            display_top_left: (0,0),
            display_bottom_right: (0,0), //TODO: change to max?
            map_data: Vec::new(),
            display_area: Vec::new(),
            lib_recv: recv,
        }
    }

    fn process_commands(&mut self) {
        use self::MapCommand::*;

        let mut iter = self.lib_recv.iter();
        while let Some(c) = iter.next() {
            match c {
                SetDisplayArea(tl, br, v) => {
                    for row in &mut self.display_area[tl.1..br.1] {
                        for c in &mut row[tl.0..br.0] {
                            *c = v;
                        }
                    }
                },
                ToggleDisplayArea(tl, br) => {
                    for row in &mut self.display_area[tl.1..br.1] {
                        for c in &mut row[tl.0..br.0] {
                            *c = !(*c);
                        }
                    }
                },
                Display(tl, br)         => {
                    self.display_top_left = tl;
                    self.display_bottom_right = br;
                },
                MapData(v)              => self.map_data = v,
            }
        }
    }
}

impl Render for Map {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord) {
        self.process_commands();
        /*let x_offset = self.display_top_left.0;
        let y_offset = self.display_top_left.1;

        for y in self.display_top_left.1..self.display_bottom_right.1 {
            for x in self.display_top_left.0..self.display_bottom_right.0 {
                let c = if self.display_area[y][x] {
                    self.map_data[y][x]
                } else {
                    ' '
                };
                w.mvaddch((y - y_offset) as i32, (x - x_offset) as i32, c);
            }
        }*/
        //TODO: rewrite
    }
}
