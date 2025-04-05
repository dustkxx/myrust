fn main () {
    let s1 = String::from("kkkkkk1231233434234");
    let len = s1.len();
    println!("s1的长度是：{}", len);
    let s2 = &s1[18..len];
    println!("this s2 is {}", s2);
}