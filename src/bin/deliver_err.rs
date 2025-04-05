use std::{fs::File, io::Read};

  

  fn main() {
//    let file_name  = read_username_from_file().unwrap();
//    println!("File name is {}", file_name);
    // let file_name_2: String  = read_username_from_file_2().unwrap();
    // println!("File name is {}", file_name_2);
    let file_name_3:Result<String, std::io::Error>  = read_username_from_file_3();
    println!("File name is {:?}", file_name_3);
  }

  // 错误传递
  fn read_username_from_file() -> Result<String, std::io::Error> {
    // the ? operator is used to propagate errors
    // if the function returns a Result, the ? operator will return the error
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string( &mut s)?;
    Ok(s)
  }

  fn read_username_from_file_2() -> Result<String, std::io::Error> {
    let  f = File::open("hello.txt");
    let mut f = match  f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
  }

  fn read_username_from_file_3()->Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
  }