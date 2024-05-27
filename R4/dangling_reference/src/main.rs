fn main() {
    let reference_to_nothing = dangling();
}
fn dangling()-> &String{
    let s = String::from("Hello");
    &s
}
