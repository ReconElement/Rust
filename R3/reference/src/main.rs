//mutable references
fn main(){
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2=&s;
    println!("{r1}  {r2}");//scope of these immutable references ends here, they're called here in the code for the last time
    let r3 = &mut s;
    println!("{r3}");
}
