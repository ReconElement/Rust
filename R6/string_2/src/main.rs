fn main() {
    //let mut name = "Omkar".to_string();
    let mut name = String::from("Omkar");
    name.push_str(" Panda");
    println!("{}",name);
    println!("{}",name.len());
    name.push(' ');
    println!("{}",name.len());
}
