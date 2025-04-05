
use std::fs::File;

fn file_open() {
    let file = File::open("move.rs");
    match file {
        Ok(_ ) => println!("File   opened successfully"),
        Err(e) => println!("Error opening file: {}", e),
    }
}

fn unwrap_file_open() {
    let file = File::open("move.rs").unwrap();
    println!("File  opened successfully");
}

fn expect_file_open() {
    let file = File::open("move.rs").expect("文件打开失败");
    println!("File  opened successfully");
}

fn main() {
    // file_open();
    // unwrap_file_open();
    expect_file_open();
} 