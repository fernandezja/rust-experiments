//Play on rust playgraound:
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7e61155c9ba327a766124b8ebc40898f
fn main() {
    let message = get_message_the_force();
    println!("Hello World! {}", message);
}

fn get_message_the_force() -> &'static str  {
    let message = "The force is strong with you!";
    return message;
}

