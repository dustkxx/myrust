// Vector

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3, 4, 5];
    // push v1 to v
    v.extend(&v1);
    // iterate v
    for i in &v {
        println!("{}", i);
    }
    // get by get mothod, if the index is out of range, it will return None
    let first = v.get(6);
    match first {
        Some(value) => println!("The first element is {}", value),
        None => println!("element not found"),
    }

    // get by index,if the index is out of range, it will panic
    // let some_by_index = &v[100];
    // println!("The first element is {}", some_by_index);
    the_borrow_of_evctor();
    


}

fn the_borrow_of_evctor() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("The first element is {}", first);
    v.push(6);
    // println!("The first element is {}", first);
  
}
