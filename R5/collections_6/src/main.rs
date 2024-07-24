fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 1..=10{
        v.push(i);
    }
    let third: &i32=&v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third{
        Some(third)=>println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    newfunc();
}

fn newfunc(){
    let v = vec![1,2,3,4,5,6,7];
    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
