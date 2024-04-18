use std::io;
fn main() {
    println!("🌡️ Converter:");
    println!("1. Fahrenheit to Celsius  2.Celsius to Fahrenheit");

    let mut input = String::new();
    let mut temperature = String::new();
    let mut result = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid number");

    let input = input.trim();

    if input == "1"{
        println!("Please enter the Fahrenheit temperature:");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Please enter a valid number");

        match temperature.trim().parse::<f64>(){
            Ok(num)=>{
                convert_to_celsius(num);
            },
            Err(_,)=>{
                println!("Please enter a valid number");
            }
        }
    }else if input == "2" {
        println!("Please enter the Celsius temperature:");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        match temperature.trim().parse::<f64>(){
            Ok(num)=>{
                convert_to_fahrenheit(num);
            },
            Err(_,)=>{
                println!("Please enter a valid integer number");
            }
        }
    }

}

fn convert_to_celsius( temperature : f64) {
    let celsius = (temperature - 32.0) / 1.8;
    println!("{}°F is {}°C", temperature, celsius);
}

fn convert_to_fahrenheit(temperature : f64) {
    let fahrenheit = (temperature  * 1.8) + 32.0;
    println!("{}°C is {}°F", temperature, fahrenheit);
}