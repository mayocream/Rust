fn main() {
    let mut s = String::new();

    s.push_str("foo");
    s.push_str("bar");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // iterating over strings
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
