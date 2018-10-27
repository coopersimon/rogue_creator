// Extension libraries used for internal engine things

use Coord;
use modscript::{Value, VType, Error, Type, RunCode};

pub mod math;
pub mod txtrend;

// Converts objects/pairs to Coord
fn to_coord(val: &Value) -> Result<Coord, Error> {
    use self::Value::*;
    use self::VType::*;

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
            let x = match x_val {
                Val(I(i))   => i.clone() as usize,
                Val(F(f))   => f.round() as usize,
                Ref(ref r)  => match *r.borrow() {
                    I(i)    => i as usize,
                    F(f)    => f.round() as usize,
                    _       => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                }
                _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
            };
            let y = match y_val {
                Val(I(i))   => i.clone() as usize,
                Val(F(f))   => f.round() as usize,
                Ref(ref r)  => match *r.borrow() {
                    I(i)    => i as usize,
                    F(f)    => f.round() as usize,
                    _       => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
                }
                _           => return Err(Error::new(Type::RunTime(RunCode::TypeError))),
            };
            Ok((x, y))
        },
        _           => Err(Error::new(Type::RunTime(RunCode::TypeError))),
    }
}
