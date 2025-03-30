#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
    
}
impl Rectangle{
    // associated function
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    // method
    // method is a function that is defined within the context of a struct
    // method can be called on an instance of the struct like rect.area()
    pub fn can_hold(&self, other_ract: &Rectangle) -> bool {
        (self.width > other_ract.width && self.height > other_ract.height) 
        || (self.width > other_ract.height && self.height > other_ract.width)
    }
    // associated function 
    // associated function is often used to create a new instance of the struct
    // associated function can be called on the struct itself like Rectangle::square(10)
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    
}
pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}




