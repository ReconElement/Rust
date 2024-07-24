use std::io;
fn main(){
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Cannot read value");
    let num: u32 = s.trim().parse().expect("Invalid input");
    println!("{num}, {num}+3 = {}",num+3);
}
