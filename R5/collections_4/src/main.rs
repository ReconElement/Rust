use std::collections::btree_map::BTreeMap;
//A client at a bar, they have their blood alcohol levels
struct Person{
    blood_alcohol:f32
}
fn main() {
    //All the orders made to the bar, by client id
    let orders = vec![2,1,2,3,4,1,2,2,3,4,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    //Our clients
    let mut blood_alcohol= BTreeMap::new();
    for id in orders{
        //If this is the first time we've seen a person initialize them with no blood alcohol, otherwise just retrieve them
        let person = blood_alcohol.entry(id).or_insert(Person{blood_alcohol:0.0});
        //Reduce their blood alcohol level, it takes time to order and drink a beer
        person.blood_alcohol*=0.9;
        //Check if they're sober enough to drink another beer
        if person.blood_alcohol>0.3{
            //Too drunk for now
            println!("Sorry {id} I have to cut you off!");
        }
        else{
            //have another
            person.blood_alcohol*=0.1;
        }
    }
}
