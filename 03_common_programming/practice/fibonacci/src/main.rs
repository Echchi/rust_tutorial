use std::io;
fn main() {
    loop{
        println!("Which Fibonacci number would you like to generate? ✏️");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid number");

        match input.trim().parse::<i32>() {
            Ok(index)=>{
                let result = fibonacci_generator(index);
                println!("The {}th Fibonacci number is {}", index, result)
            },
            Err(_)=>{
                println!("Please enter a valid number");
            }
        }
    }
}

fn fibonacci_generator(index : i32) -> i32{
    let mut a = 0;
    let mut b = 1;
    let mut i = 1;

    if index == 0 {
        return a;
    }

    while i < index {
     let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }

    b
}
