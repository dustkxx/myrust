mod str_utils;
mod first_struct;
mod tuplestruct;


use str_utils::split_words;
use first_struct::User;
use first_struct::build_user;

    // split_words with spaces

    // fn main() {
    //     let mut s = String::new();
    //     println!("请输入一个字符串:");
    //     std::io::stdin().read_line(&mut s).unwrap();
    //     let words = split_words(&s);
    //     for (i, &item) in words.iter().enumerate() {
    //         println!("word {}: {}", i, item);
    //     }
    // }
    
    // fist struct User test

    // fn main(){
    //     let mut ming = build_user(String::from("ming"), String::from("中华"));
    //     let name = first_struct::get_user_name(&ming);
    //     println!("xiaohua's name is: {}", name);

    //     let mut xiaohua = User {
    //         name: String::from("xiaohua"),
    //         email: String::from("zhonghua"),
    //         sign_in_count: 0,
    //         age: 0,
    //         active: false,
    //     };
    //     // update xiaohua's name
    //     xiaohua = User {
    //         name: String::from("hhhhhhhh"),
    //         ..xiaohua
    //     };
    //     println!("User's name is : {}", xiaohua.name);

    // }


    // struct mothod and associated function test
    
    // fn main() {
    //     let mut rect = tuplestruct::Rectangle {
    //         width: 30,
    //         height: 50,
    //     };
    //     println!("rect's area is : {}", rect.area());
    //     rect.width = 20;
    //     println!("rect's area is : {}", tuplestruct::area(&rect));
    //     println!("rect's area is : {:#?}", rect);
    //     println!("rect's area is : {:#?}", rect);

    //     let rect1 = tuplestruct::Rectangle {
    //         width: 10,
    //         height: 20,
    //     };
    //     let rect2 = tuplestruct::Rectangle {
    //         width: 11,
    //         height: 9,
    //     };
    //     let can_hold = rect1.can_hold(&rect2);
    //     println!("rect1 can hold rect2: {}", &can_hold);
    //     let can_hold = rect2.can_hold(&rect1);
    //     println!("rect2 can hold rect1: {}", &can_hold);

    //     // test square
    //     let square = tuplestruct::Rectangle::square(10);
    //     println!("square's area is : {}", square.area());
    //     println!("square's area is : {:#?}", square);
        

    // }