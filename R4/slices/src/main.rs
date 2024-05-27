fn main(){
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    print_slices(&s);
    s.clear();
    println!("{word}");
}
fn first_word(s: &String)->usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
fn print_slices(s: &String){
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}");
    println!("{world}");
}
