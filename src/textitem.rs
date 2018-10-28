pub enum TextColour {
    Default,
    Black,
    White,
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Grey,
}

impl TextColour {
    pub fn from_str(col: &str) -> Option<Self> {
        use self::TextColour::*;
        match col {
            "default"   => Some(Default),
            "black"     => Some(Black),
            "white"     => Some(White),
            "red"       => Some(Red),
            "blue"      => Some(Blue),
            "green"     => Some(Green),
            "yellow"    => Some(Yellow),
            "orange"    => Some(Orange),
            "purple"    => Some(Purple),
            "grey"      => Some(Grey),
            _           => None,
        }
    }
}

pub enum TextOption {
    Bold,
    Blinking,
}

impl TextOption {
    pub fn from_str(col: &str) -> Option<Self> {
        use self::TextOption::*;
        match col {
            "bold"      => Some(Bold),
            "blinking"  => Some(Blinking),
            _           => None,
        }
    }
}

pub struct TextItem {
    pub text: String,
    pub colour: TextColour,
    pub options: Vec<TextOption>,
}

impl TextItem {
    pub fn new(text: String, len: Option<usize>) -> Self {
        let mut item = TextItem {
            text: text,
            colour: TextColour::Default,
            options: Vec::new(),
        };

        match len {
            Some(l) => item.text.truncate(l),
            None    => (),
        };

        item
    }
}
