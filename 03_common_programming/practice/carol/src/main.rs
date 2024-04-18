use std::io;
fn main() {
    loop {
        println!("Do you want to know the lyrics of 12 days of Christmas? 🎅");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Please answer y or n");

        let input = input.trim().to_lowercase();

        if input == "y" || input == "yes"{
            sing_carol();
        }else if input == "n" || input == "no" {
            println!("Okay... Bye... 🎅");
        }else{
            println!("Please answer y/n or yes/no")
        }
    }
}

fn sing_carol(){
    let nth = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let month_lyrics = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for i in 0..12 {
        if i == 0 {
            println!("🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶");
        }
        println!("On the {} day of Christmas,", nth[i]);
        println!("my true love gave to me");
        for j in (0..=i).rev() {
            if i == 0 {
                println!("{}",month_lyrics[j]);
            }else if j == 0{
                println!("And {}",month_lyrics[j]);
            }else {
                println!("{}",month_lyrics[j]);
            }
        }
        println!("");
        if i == 11 {
            println!("🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶🎶");
        }
    }
}
