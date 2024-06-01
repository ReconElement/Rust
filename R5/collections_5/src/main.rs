use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
#[derive(Debug)]
struct Foo{
    a: u32,
    b: &'static str
}
//we will compare foo by their 'a' value only
impl PartialEq for Foo{
    fn eq(&self, other: &Self) -> bool{self.a == other.a}
}
impl Eq for Foo{}
//we will hash `Foo`s by their `a` value only
impl Hash for Foo{
    fn hash<H: Hasher>(&self, h: &mut H){
        self.a.hash(h);
    }
}
impl PartialOrd for Foo{
    fn partial_cmp(&self, other: &Self) -> Ordering{self.a.cmp(&other.a)}
}
impl Ord for Foo{
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a)
    }
}
fn main() {
    let mut map = BTreeMap::new();
    map.insert(Foo{a: 1, b: "baz"},99);
    //We already have a foo with an o of 1 so this will be updating the value
    map.insert(Foo{a:1, b:"xyz"},100);
    //The value has been updated
    assert_eq!(map.values().next().unwrap(),&100);
    //but the keys have not changed b is still "baz" and not "xyz"
    assert_eq!(map.keys().next().unwrap().b,"baz");
}
