#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn parameter(&self)->u32{
        2*(self.height+self.width)
    }
}
fn main() {
    let rec1 = Rectangle{
        width: 50,
        height: 50
    };
    println!("The parameter and area of the rectangle is: {} {}",rec1.parameter(),rec1.area());
}
