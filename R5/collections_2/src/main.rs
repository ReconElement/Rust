use std::collections::{vec_deque, VecDeque};
fn main() {
    let mut vec1=vec![1,2,3,4,5,6];
    let vec2 = vec![10,20,30,40,50];
    vec1.extend(vec2);
    for x in vec1.iter(){
        println!("{x}");
    }
    let vec = [1,2,3,4,5,6];
    let buf: VecDeque<_> = vec.into_iter().collect();
    for x in vec1.iter().rev(){
        println!("{x:?}");
    }
}
