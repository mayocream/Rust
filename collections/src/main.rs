#![allow(unused)]

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // reading elements of a vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // None

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // println!("The first element is: {}", first);
    // 为什么第一个元素的引用会关心 vector 结尾的变化？不
    // 能这么做的原因是由于 vector 的工作方式：
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

    // iterating over the values in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // using an enum to store multiple types
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
