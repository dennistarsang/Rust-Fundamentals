extern crate Phrases;  


fn main() {
    println!("English: {}, {}",
        Phrases::greetings::english::hello(),
        Phrases::greetings::english::goodbye()
    );
}
