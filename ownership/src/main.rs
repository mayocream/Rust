#![allow(unused)]

fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}, world!", s1);

    // Ownership and Functions
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // This will not work

    let x = 5;

    makes_copy(x);

    println!("{}", x);


    // Return Values and Scope
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s1);

    // references and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &mut s;
    // let r2 = &mut s; // This will not work
    println!("{}", r1);

    

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
