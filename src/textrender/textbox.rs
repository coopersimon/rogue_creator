use Coord;
use super::Render;
use pancurses::Window;
use textitem::TextItem;

pub struct TextBox {
    text: TextItem,
}

impl TextBox {
    pub fn new(text: TextItem) -> Self {
        TextBox {
            text: text,
        }
    }
}

impl Render for TextBox {
    fn render(&mut self, w: &mut Window, top_left: Coord, bottom_right: Coord) {
        // TODO: split into lines
        let length = bottom_right.0 - top_left.0;

        w.mvaddnstr(top_left.1 as i32, top_left.0 as i32, &self.text.text, length as i32);
    }
}
