// Extension libraries used for internal engine things

use Coord;
use textitem::{TextItem, TextColour, TextOption};
use modscript::{Value, VType, Error, Type, RunCode};

pub mod math;
pub mod txtrend;
pub mod glob;
pub mod level;
pub mod entity;
pub mod pbox;
pub mod map;
pub mod makemap;
pub mod control;

// Converts objects/pairs to Coord
fn to_coord(val: &Value) -> Result<Coord, Error> {
    use self::Value::*;
    use self::VType::*;

    fn make_coord(x_val: &Value, y_val: &Value) -> Result<Coord, Error> {
        let x = match x_val {
            Val(I(i))   => i.clone() as usize,
            Val(F(f))   => f.round() as usize,
            Ref(ref r)  => match *r.borrow() {
                I(i)    => i as usize,
                F(f)    => f.round() as usize,
                _       => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
            },
            _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
        };
        let y = match y_val {
            Val(I(i))   => i.clone() as usize,
            Val(F(f))   => f.round() as usize,
            Ref(ref r)  => match *r.borrow() {
                I(i)    => i as usize,
                F(f)    => f.round() as usize,
                _       => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
            },
            _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
        };
        Ok((x, y))
    }

    match val {
        Obj(ref o)  => {
            let o = o.borrow();
            let x_val = match o.get("x") {
                Some(x) => x,
                None    => return Err(Error::new(Type::RunTime(RunCode::TypeError))), // Better error?
            };
            let y_val = match o.get("y") {
                Some(y) => y,
                None    => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
            };
            make_coord(x_val, y_val)
        },
        List(ref l) => {
            let l = l.borrow();
            if l.len() < 2 {
                return Err(Error::new(Type::RunTime(RunCode::TypeError)));
            }
            make_coord(&l[0], &l[1])
        },
        _           => Err(Error::new(Type::RunTime(RunCode::TypeError))),
    }
}

fn to_text_item(val: &Value) -> Result<TextItem, Error> {
    use self::Value::*;
    use self::VType::*;

    match val {
        Str(ref s)  => {
            Ok(TextItem::new(s.borrow().clone(), None))
        },
        Obj(ref o)  => {
            let o = o.borrow();
            let text = match o.get("text") {
                Some(t) => match t {
                    Str(ref s)  => s.borrow().clone(),
                    _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                },
                None    => return Err(Error::new(Type::RunTime(RunCode::TypeError))), // Better error?
            };
            let len = match o.get("len") {
                Some(l) => match l {
                    Val(I(i))   => Some(i.clone() as usize),
                    Ref(ref r)  => match *r.borrow() {
                        I(i)    => Some(i as usize),
                        _       => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                    },
                    _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                },
                None    => None,
            };

            let mut textitem = TextItem::new(text, len);

            match o.get("colour") {
                Some(c) => match c {
                    Str(ref s)  => textitem.colour = TextColour::from_str(&*s.borrow()).unwrap(), // TODO: better error
                    _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                },
                None    => (),
            };

            match o.get("options") {
                Some(op) => match op {
                    Str(ref s)  => textitem.options.push(TextOption::from_str(&*s.borrow()).unwrap()),
                    List(ref l) => {
                        let l = l.borrow();
                        for i in l.iter() {
                            match i {
                                Str(ref s)  => textitem.options.push(TextOption::from_str(&*s.borrow()).unwrap()),
                                _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                            }
                        };
                    },
                    _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                },
                None    => (),
            };

            Ok(textitem)
        },
        _           => Err(Error::new(Type::RunTime(RunCode::TypeError))),
    }
}
