fn main() {
    let mut s1 = String::from("hello");

    change_s1(&mut s1);

    println!("{}", s1);
}

fn change_s1(some_string: &mut String) {
    some_string.push_str(", world");
}
