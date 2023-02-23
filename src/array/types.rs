
pub enum Type{
    Str(String),
    Int(isize),
    Float(f64),
    Bool(bool),
}


impl Type{
    pub fn get_type(&self) -> &'static str{
        match self {
            Self::Str(_) => "str",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
        }
    }

    pub fn unwrap_float(&self) -> f64{
        if let Self::Float(i) = self{
            *i
        }else{
            panic!("is not float")
        }
    }
    pub fn unwrap_bool(&self) ->  bool{
        if let Self::Bool(i) = self{
            *i
        }else{
            panic!("is not boolean")
        }
    }
    pub fn unwrap_string(&self) -> String{
        if let Self::Str(i) = self{
            i.to_string()
        }else{
            panic!("is not String")
        }
    }
    pub fn unwrap_str(&self) -> &str{
        if let Self::Str(i) = self{
            i
        }else{
            panic!("is not str")
        }
    }
}

pub trait Unwrap_int<T>{
    fn unwrap_int(&self) -> T;
}
macro_rules! type_add {
    ($($x: tt) *) => {
        $(
            impl Unwrap_int<$x> for Type{
                fn unwrap_int(&self) -> $x{
                    if let Self::Int(i) = self{
                        *i as $x
                    }else{
                        panic!("is not integer")
                    }
                }
            }
        )*
    };
}

type_add![isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128];



impl std::fmt::Display for Type{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.get_type() {
            "str" => format!("{:?}", self.unwrap_str()),
            "int" => {
                let a: isize = self.unwrap_int();
                format!("{a}")
            },
            "float" => format!("{:?}", self.unwrap_float()),
            "bool" => format!("{:?}", self.unwrap_bool()),
            _ => "Null".to_string()
        })
    }
}

impl std::fmt::Debug for Type{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.get_type() {
            "str" => format!("Str({:?})", self.unwrap_str()),
            "int" => {
                let a: isize = self.unwrap_int();
                format!("Int({a:?})")
            },
            "float" => format!("Float({:?})", self.unwrap_float()),
            "bool" => format!("Bool({:?})", self.unwrap_bool()),
            _ => "Null".to_string()
        })
    }
}

