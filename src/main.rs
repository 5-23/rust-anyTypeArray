use crate::array::array::{Vector, Array};

mod array;


fn main() {
    let mut a = array![0 10. "asdf"];
    let b = array![0, 10., "asdf"];
    a.push(true);
    println!("{}, {}", a, b);

    for i in a.iter(){
        println!("{i:?}");
    }
}