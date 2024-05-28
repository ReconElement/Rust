#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}
impl Rectangle{
    fn area(&self)->u32{
        self.height*self.width
    }
    fn can_hold(&self, other: &Rectangle)->bool{
        self.height>other.height && self.width>other.width
    }
    //associated function
    fn square(side: u32)->Self{
        Self{
            height: side,
            width: side
        }
    }
}
fn main() {
    let rect1 = Rectangle{
        height: 50,
        width: 80
    };
    let rect2 = Rectangle{
        height: 56,
        width: 83
    };
    let rec3 = Rectangle{
        height: 30,
        width: 40
    };
    println!("Can the first rectangle hold the third rectangle: {}",rect1.can_hold(&rec3));
    println!("Can the first rectangle hold the second rectangle {}",rect1.can_hold(&rect2));
    let side = 5;
    let square = Rectangle::square(5);
    println!("The dimensions of the square are: {:#?}",square);
}
