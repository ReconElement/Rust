fn main(){
    let s1 = String::from("Hello");
    let s3 = calculate_length_ref(&s1);
    let (s2, length) = calculate_length(s1);
    println!("{s2} {length}");
    println!("{s3}");
}
fn calculate_length(s: String)->(String, usize){
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String)->usize{//s is a reference to a string
    s.len()
}
