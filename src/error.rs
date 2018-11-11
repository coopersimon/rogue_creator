use modscript::{Error, CustomError, Type};

pub enum EngErr {
    RunTime(RunCode),
}

pub fn engerr(code: EngErr) -> Error {
    Error::new(Type::Package(Box::new(code)))
}

impl CustomError for EngErr {
    fn to_string(&self) -> String {
        use self::EngErr::*;
        match *self {
            RunTime(ref c)  => format!("Internal runtime error: {:?}", c),
        }
    }
}

#[derive(Debug)]
pub enum RunCode {
    EntityClassNotFound,
    LevelClassNotFound,
}
