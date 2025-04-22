fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    // let result = longest(string1.as_str(), string2.as_str());
    let result = longest_1(string1, string2);
    println!("The longest string is: {}", result);
}

//// 1. 生命周期参数
//// 2. 生命周期参数的使用
//// 3. 生命周期参数的约束
fn  longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    
}

fn  longest_1(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    
}