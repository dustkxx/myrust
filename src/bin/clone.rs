fn main (){
    let s = String::from("hello");
    // the s1 is a clone of s
    // s has not moved
    let s1 = s.clone();
    print!("{}", s);
    println!("{}", s1);
}