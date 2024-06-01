fn main() {
   let dice_roll= 9;
    match dice_roll{
        3=>add_fancy_hat(),
        7=>remove_fancy_hat(),
        other=>move_player(other)
    }
    let config_max=Some(3u8);
    match config_max{
        Some(max)=>println!("The maximum is configured to be {max}"),
        _=>()
    }
    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    }
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_spaces: u8){}