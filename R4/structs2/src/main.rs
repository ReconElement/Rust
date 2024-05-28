fn main() {
    let rec1 = Rectangle{
        width: 50,
        height: 50
    };
    println!("The Rectangle is of dimensions: {:#?}",rec1);
    println!("The area of the rectangle is {}",area(&rec1));
}
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
}
