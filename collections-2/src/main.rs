fn main() {

    let mut v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("Print third element {third}");

    let third: Option<&i32> = v.get(20);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("out of bounds, vec max length is {}", v.len())
    }

    for i in &mut v {
        *i += 40;
    }
    dbg!(v);

}
