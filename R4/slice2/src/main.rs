use std::io;
fn main() {
    println!("Enter a few words to find the first word: ");
    let mut s= String::new();
    io::stdin().read_line(&mut s).expect("Cannot read the string");
    let first_word = first_word(&s);
    println!("The first word is {first_word}");
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}
fn first_word(s: &String)-> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
