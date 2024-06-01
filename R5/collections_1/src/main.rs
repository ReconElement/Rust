fn main() {
    let vec = vec![1,2,3,4,5,6];
    for x in vec.iter(){
        println!("vec contains {x:?}");
    }

    let mut vecc=vec![1,2,3,4,5,6];
    for x in vecc.iter_mut(){
        *x=*x+1;
    }
    for x in vecc.iter(){
        println!("vecc contains after change {x:?}")
    }
}
