fn main() {
    let s = String::from("hello");
    // the borrowed s is moved to s1
    let s1 = s;
    // println!("{}", s); // error: value moved
    print!("{}", s);
}
