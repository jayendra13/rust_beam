#![allow(dead_code, unused_variables)]

struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional_num: Option<i32>
}

struct TwoNums(i32, i32);

impl Data {

    fn new() -> Self {
        Data {
            num1: 3,
            num2: 4,
            str1: "some string".to_string(),
            optional_num: Some(8)
        }
    }

    fn sum(&self) -> i32 {
        self.num1 + self.num2
    }
}

fn main() {

    let a = Data {
        num1: 4,
        num2: 2,
        str1: "whatever".to_string(),
        optional_num: None
    };

    println!("{}", a.sum());

    let b = Data{num1: 8, ..a};
    println!("{}", b.sum());

    let mut c = Data::new();
    c.num1 = 3;
    println!("{}", c.sum());


    let d = TwoNums(4, 3);
    println!("{}, {}", d.0, d.1);
    // println!("{:?}", a);
}