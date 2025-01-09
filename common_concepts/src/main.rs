#[allow(unused_variables)]

fn main() {
    // 3.1 variables and mutability
    let mut x = 5;
    x = 6;
    println!("The value of x is: {}", x);

    // 3.2 data types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {:?}, y: {:?}", x, y);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
    println!("tup.0: {:?}, tup.1: {:?}, tup.2: {:?}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];

    // 3.3 functions
    another_function(5);

    let x = five();
    println!("The value of x is: {}", x);

    // 3.4 comments
    // this is a line comment

    // 3.5 control flow
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter().rev() {
        println!("the value is: {}", element);
    }

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
