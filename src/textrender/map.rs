use pancurses::Window;

use super::Render;
use Coord;
use tile::{TileInfo, TileID};
use textitem::TextItem;

use std::rc::Rc;
use std::sync::mpsc::Receiver;

// TODO: less copying here (rc?)
pub enum MapCommand {
    Display(Coord, Coord),
    SetDisplayArea(Coord, Coord, bool),
    ToggleDisplayArea(Coord, Coord),
    TileInfo(Rc<TileInfo>),
    TileData(Vec<Vec<TileID>>),
    Sprite(Coord, TextItem),
}

pub struct Map {
    display_top_left: Coord,
    display_bottom_right: Coord,
    display_area: Vec<Vec<bool>>,
    tile_info: Rc<TileInfo>,
    tile_data: Vec<Vec<TileID>>,
    instance_data: Vec<(Coord, TextItem)>,
    lib_recv: Receiver<MapCommand>,
}

impl Map {
    pub fn new(recv: Receiver<MapCommand>) -> Self {
        Map {
            display_top_left: (0,0),
            display_bottom_right: (0,0), //TODO: change to max?
            display_area: Vec::new(),
            tile_info: Rc::new(TileInfo::new()),
            tile_data: Vec::new(),
            instance_data: Vec::new(),
            lib_recv: recv,
        }
    }

    fn process_commands(&mut self) {
        use self::MapCommand::*;

        let mut iter = self.lib_recv.try_iter();
        while let Some(c) = iter.next() {
            match c {
                Display(tl, br)         => {
                    self.display_top_left = tl;
                    self.display_bottom_right = br;
                },
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
                TileInfo(ti)    => self.tile_info = ti,
                TileData(td)    => self.tile_data = td,
                Sprite(c, ti)   => self.instance_data.push((c, ti)),
            }
        }
    }

    fn in_range(&self, c: Coord) -> bool {
        (c.0 >= self.display_top_left.0)        &&
        (c.0 <  self.display_bottom_right.0)    &&
        (c.1 >= self.display_top_left.1)        &&
        (c.1 <  self.display_bottom_right.1)
    }
}

impl Render for Map {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord) {
        self.process_commands();

        let default_tile = TextItem::new_tile(' '.to_string());

        for y in self.display_top_left.1..self.display_bottom_right.1 {
            for x in self.display_top_left.0..self.display_bottom_right.0 {
                let text = if self.display_area[y][x] {
                    &self.tile_info.get_item(self.tile_data[y][x]).unwrap().text
                } else {
                    &default_tile
                };

                if ((top_left.1 + y) <= bottom_right.1) && ((top_left.0 + x) <= bottom_right.0) {
                    w.mvaddch((top_left.1 + y) as i32, (top_left.0 + x) as i32, text.as_char());
                }
            }
        }

        for &(ref c, ref t) in self.instance_data.iter() {
            if self.in_range(*c) {
                let text = if self.display_area[c.1][c.0] {
                    &t
                } else {
                    &default_tile
                };

                if ((top_left.1 + c.1) <= bottom_right.1) && ((top_left.0 + c.0) <= bottom_right.0) {
                    w.mvaddch((top_left.1 + c.1) as i32, (top_left.0 + c.0) as i32, text.as_char());
                }
            }
        }
    }
}
