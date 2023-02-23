use super::types::Type;

pub struct Array{
    v: Vec<Type>
}

pub trait Vector<T>{
    fn push(&mut self, item: T);
    fn insert(&mut self, index: usize, item: T);
}

impl Array{
    pub fn new() -> Self{
        Self{
            v: vec![]
        }
    }
    pub fn remove(&mut self, index: usize){
        self.v.remove(index);
    }

    pub fn sort(&mut self, key: impl Fn(&Type) -> isize){
        self.v.sort_by_key(key);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Type>{
        self.v.iter()
    }
}


impl std::ops::Index<usize> for Array{
    type Output = Type;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}







impl Vector<&'static str> for Array{
    fn push(&mut self, item: &'static str) {
        self.v.push(Type::Str(item.to_string()))
    }
    fn insert(&mut self, index: usize, item: &'static str){
        self.v.insert(index, Type::Str(item.to_string()));
    }
}
impl Vector<String> for Array{
    fn push(&mut self, item: String) {
        self.v.push(Type::Str(item))
    }
    fn insert(&mut self, index: usize, item: String){
        self.v.insert(index, Type::Str(item));
    }
}

impl Vector<bool> for Array{
    fn push(&mut self, item: bool) {
        self.v.push(Type::Bool(item))
    }
    fn insert(&mut self, index: usize, item: bool){
        self.v.insert(index, Type::Bool(item));
    }
}

impl Vector<f32> for Array{
    fn push(&mut self, item: f32) {
        self.v.push(Type::Float(item as f64))
    }
    fn insert(&mut self, index: usize, item: f32){
        self.v.insert(index, Type::Float(item as f64));
    }
}

impl Vector<f64> for Array{
    fn push(&mut self, item: f64) {
        self.v.push(Type::Float(item))
    }
    fn insert(&mut self, index: usize, item: f64){
        self.v.insert(index, Type::Float(item));
    }
}

macro_rules! integers {
    ($($x: tt) *) => {
        $(impl Vector<$x> for Array{
            fn push(&mut self, item: $x) {
                self.v.push(Type::Int(item as isize))
            }
            fn insert(&mut self, index: usize, item: $x){
                self.v.insert(index, Type::Int(item as isize));
            }
        })*
    };
}
integers![isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128];


impl std::fmt::Display for Array{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rmx = String::new();
        let mut ind = 0;
        for i in &self.v{
            ind += 1;
            if self.v.len() == ind{
                rmx.push_str(&format!("{i}"));
            }else{
                rmx.push_str(&format!("{i}, "));
            }
        }
        write!(f, "[{}]", rmx)
    }
}

impl std::fmt::Debug for Array{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.v)
    }
}


/**
array![0, 10., "asdf"]
array![0 10. "asdf"]
*/
#[macro_export]
macro_rules! array {
    [] => {
        vec![]
    };
    [$($x: tt), *] => {
        {
            let mut arr = Array::new();
            $(
                arr.push($x);
            )*
            arr
        }
    };
    [$($x: tt) *] => {
        {
            let mut arr = Array::new();
            $(
                arr.push($x);
            )*
            arr
        }
    };
}