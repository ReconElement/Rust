fn main(){
    let mut s = String::new();
    let data = "Initial contents";
    let s = "Initial contents".to_string();
    let s_ = String::from("Initial Contents");
    //let s = data.to_string();
    //let s = String::from(data);
    println!("{s}");
    let namaste = "नमस्ते";
    for i in namaste.chars(){
        println!("{}",i);
    }
}
