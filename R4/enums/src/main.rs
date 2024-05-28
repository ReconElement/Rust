fn main() {
    let four = IpAddrKind::V4(1,1,1,1);
    let six = IpAddrKind::V6(String::from("::1"));
    let absent_number: Option<i32>=None;
    let present_number= Some(6);
}
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String)
}



