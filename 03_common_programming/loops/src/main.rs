fn main() {
    // 러스트 반복문 : loop, while, for
    // infinite_loop()
    res_loop(0);
    multi_loop();
    while_fn();
    while_arr_fn();
    for_fn();
    rev_fn();
}

// loop
fn infinite_loop(){
    loop{
        println!("💕💕💕💕💕💕💕💕💕💕💕💕💕💕")
    }
}

fn res_loop (mut counter : i32) {
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 5;
        }
    };

    println!("The result is {result}")
}

fn multi_loop (){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_fn(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("🚀");
}

fn while_arr_fn(){
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value in while_arr_fn is {}", arr[index]);

        index += 1;
    }

}

fn for_fn(){
    let arr = [10, 20, 30, 40, 50];
    
    for element in arr {
        println!("The value in for_fn is {element}");
    }
}

fn rev_fn(){
    for number in (1..4).rev(){
        println!("rev {number}!");
    }
    println!("🚀")
}
