fn main() {
    let some_num: Option<i32> = Option::Some(5);
    let some_str: Option<&str> = Option::Some("World");
    let null_num_op: Option<i32> = Option::None;
    let null_str_op: Option<&str> = Option::None;

    println!("the result num is {}", add(6, some_num));
    println!("the result num is {}", add(6, null_num_op));
    println!("the result String is {}", concat("Hello", some_str));
    println!("the result String is {}", concat("Hello", null_str_op));

    concat_useif("hello", some_str);
    concat_useif("hello", null_str_op);
}

fn add(num: i32, op: Option<i32>) -> i32 {
    // option 为空则直接返回num，否则数值相加
    match op {
        Some(value) => {
            println!("解析到op的值为：{}", value);
            value + num
        }
        None => num,
        // 由于match表达式必须要枚举的所有类型，如果无法列举所有情况，其他所有的情况 使用下划线表示 _   例如上行代码可以写为
        // _ => num,
    }
}

fn concat(str: &str, op: Option<&str>) -> String {
    // option 为空则直接返回num，否则数值相加
    match op {
        Some(value) => format!("{},{}", str, value),
        None => {
            println!("Op中没有值，无法进行字符连接");
            str.to_string()
        }
    }
}

fn concat_useif(str: &str, op: Option<&str>) {
    // 使用 if let 句式去匹配结果
    if let Some(value) = op {
        println!("The value is: {},{}", str, value);
    } else {
        println!("No value found");
    }

    if let Some(value) = op {
        println!("The value is: {},{}", str, value);
    } else if let None = op {
        println!("No value found");
    }
}
