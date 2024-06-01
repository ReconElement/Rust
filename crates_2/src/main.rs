fn deliver_order(){}

mod back_of_house{
    fn cook_order(){}
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
}
fn main() {
    let s:Option<i32>=Some(32);
}
