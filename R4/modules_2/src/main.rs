mod my{
    //A public struct with a public field of generic type 'T'
    pub struct OpenBox<T>{
        pub contents: T
    }
    //A public struct with a private field of generic type 'T'
    pub struct ClosedBox<T>{
        contents: T
    }
    impl<T> ClosedBox<T>{
        //a public constructor method
        pub fn new(contents: T) -> ClosedBox<T>{
            ClosedBox{
                contents
            }
        }
    }
}
fn main() {
    //Public structs with public fields can be constructed as usual
    let open_box=my::OpenBox{contents: "public information"};
    //and their fields can be normally accessed as well
    println!("The openbox contains {}",open_box.contents);
    //structs with private fields can be used with public constructors
    let closed_box=my::ClosedBox::new("Classified Information");

}
