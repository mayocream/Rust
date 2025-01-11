#![allow(unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    // Option<T> enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}
