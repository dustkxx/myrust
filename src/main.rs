mod str_utils;


use str_utils::split_words;
use str_utils::traits;
    fn main() {
        let mut s = String::new();
        println!("请输入一个字符串:");
        std::io::stdin().read_line(&mut s).unwrap();
        let words = split_words(&s);
        for (i, &item) in words.iter().enumerate() {
            println!("word {}: {}", i, item);
        }
    }
    // fn main() {
    //     let s = "hello world";
    //     let str = traits(s);
    //     println!("处理后的字符串: {}", str);
    // }
