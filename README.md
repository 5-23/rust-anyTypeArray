# rust-anyTypeArray
모든 타입을 넣을수있는 배열
```rs
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
```
