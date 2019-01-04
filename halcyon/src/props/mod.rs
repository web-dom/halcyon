use std::collections::HashMap;
use std::fmt;

pub type Props = HashMap<String, Prop>;

pub enum Prop {
    Props(Props),
    NumberI32(i32),
    String(String),
    Bool(bool),
    Fn00(Box<Fn()>),
}

impl fmt::Debug for Prop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prop::Fn00(_) => write!(f, "Fn00()"),
            Prop::Props(v) => write!(f, "{}", format!("{:?}", v)),
            Prop::NumberI32(v) => write!(f, "{}", format!("{:?}", v)),
            Prop::String(v) => write!(f, "{}", format!("{:?}", v)),
            Prop::Bool(v) => write!(f, "{}", format!("{:?}", v)),
        }
    }
}

impl PartialEq for Prop {
    fn eq(&self, other: &Prop) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

impl From<i32> for Prop {
    fn from(v: i32) -> Prop {
        Prop::NumberI32(v)
    }
}

impl From<String> for Prop {
    fn from(v: String) -> Prop {
        Prop::String(v)
    }
}

impl From<Props> for Prop {
    fn from(v: Props) -> Prop {
        Prop::Props(v)
    }
}

impl From<bool> for Prop {
    fn from(v: bool) -> Prop {
        Prop::Bool(v)
    }
}

impl<T> From<Box<T>> for Prop
where
    T: Fn() + 'static,
{
    fn from(v: Box<T>) -> Prop {
        Prop::Fn00(v)
    }
}
