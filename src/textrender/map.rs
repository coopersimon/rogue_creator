use pancurses::Window;

use super::Render;
use Coord;
use tile::{TileInfo, TileID};
use textitem::TextItem;

use std::rc::Rc;
use std::sync::mpsc::Receiver;

// TODO: less copying here (rc?)
pub enum MapCommand {
    // Init
    NewLevel(usize, usize),

    // Package commands
    Display(Coord),
    SetShowArea(Coord, Coord, bool),

    // Map data
    TileInfo(Rc<TileInfo>),
    TileData(Vec<Vec<TileID>>),
    Sprite(Coord, TextItem),
}

pub struct Map {
    display_top_left: Coord,
    show_area: Vec<Vec<bool>>,
    tile_info: Rc<TileInfo>,
    tile_data: Vec<Vec<TileID>>,
    instance_data: Vec<(Coord, TextItem)>,
    lib_recv: Receiver<MapCommand>,
}

impl Map {
    pub fn new(recv: Receiver<MapCommand>) -> Self {
        Map {
            display_top_left: (0,0),
            show_area: Vec::new(),
            tile_info: Rc::new(TileInfo::new()),
            tile_data: Vec::new(),
            instance_data: Vec::new(),
            lib_recv: recv,
        }
    }

    fn process_commands(&mut self) {
        use self::MapCommand::*;

        self.instance_data.clear();

        let mut iter = self.lib_recv.try_iter();
        while let Some(c) = iter.next() {
            match c {
                NewLevel(x, y)          => {
                    self.show_area = vec![vec![true; x]; y];
                },
                Display(tl)             => {
                    self.display_top_left = tl;
                },
                SetShowArea(tl, br, v)  => {
                    for row in &mut self.show_area[tl.1..br.1] {
                        for c in &mut row[tl.0..br.0] {
                            *c = v;
                        }
                    }
                },
                TileInfo(ti)    => self.tile_info = ti,
                TileData(td)    => self.tile_data = td,
                Sprite(c, ti)   => self.instance_data.push((c, ti)),
            }
        }
    }

    fn in_range(&self, br: &Coord, c: &Coord) -> bool {
        (c.0 >= self.display_top_left.0) &&
        (c.0 <  br.0)                    &&
        (c.1 >= self.display_top_left.1) &&
        (c.1 <  br.1)
    }
}

impl Render for Map {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord) {
        self.process_commands();

        let default_tile = TextItem::new_tile(' '.to_string());

        let y_bound = (bottom_right.1 - top_left.1) + self.display_top_left.1;
        let x_bound = (bottom_right.0 - top_left.0) + self.display_top_left.0;

        for y in self.display_top_left.1..y_bound {
            if y >= self.tile_data.len() {
                break;
            }

            for x in self.display_top_left.0..x_bound {
                if x >= self.tile_data[y].len() {
                    break;
                }

                let text = if self.show_area[y][x] {
                    &self.tile_info.get_item(self.tile_data[y][x]).unwrap().text
                } else {
                    &default_tile
                };

                // TODO: is the below condition necessary?
                if ((top_left.1 + y) <= bottom_right.1) && ((top_left.0 + x) <= bottom_right.0) {
                    w.mvaddch((top_left.1 + y) as i32, (top_left.0 + x) as i32, text.as_char());
                }
            }
        }

        for &(ref c, ref t) in self.instance_data.iter() {
            if self.in_range(&(x_bound, y_bound), c) {
                let text = if self.show_area[c.1][c.0] {
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
