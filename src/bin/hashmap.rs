use std::collections::{HashMap};


//  concat two vectors to a hashmap
// 1. Create two vectors, one for keys and one for values.
// 2. Use the zip method to combine them into an iterator of tuples.
fn zip() {
    let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];
    let map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
    println!("{:?}", map);
}

fn ownership() {
    let mut map = HashMap::new();
    let key = String::from("a");
    let value = 1;
    map.insert(&key, &value);
    // map.insert(key, value);// This would cause a compile error because key has been moved
    println!("{:?}", key); 
    println!("{:?}", map);
}

fn update_hashmap() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Update the value for key "b"
    // map.insert("b", 4);
    // or use entry API
    map.entry("c").or_insert(0);

    // Check if the value for key "b" has been updated
    println!("{:#?}", map); // Should print Some(&4)
}


fn main() {
    let mut map  = HashMap::new();
    map.insert("a", 1);
    println!("{:?}", map);

    // zip();
    // ownership();
    update_hashmap();
    
}