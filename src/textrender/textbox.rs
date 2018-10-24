use Coord;
use super::Render;
use pancurses::Window;

pub struct TextBox {
    top_left: Coord,
    bottom_right: Coord,
    text: String,
}

impl TextBox {
    pub fn new(tl: Coord, br: Coord, text: String) -> Self {
        TextBox {
            top_left: tl,
            bottom_right: br,
            text: text,
        }
    }
}

impl Render for TextBox {
    fn render(&mut self, w: &mut Window) {
        let length = self.bottom_right.0 - self.top_left.0;

        w.mvaddnstr(self.top_left.1 as i32, self.top_left.0 as i32, &self.text, length as i32);
    }
}
