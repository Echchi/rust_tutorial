fn main() {
    basic_control(3);
    multiple_control(24);
    
}

fn basic_control(number : i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn multiple_control(number : i32){
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_with_if (number : i32){
    let condition = true;
    // 변수가 가질 수 있는 타입이 오직 하나이기 때문에 컴파일 에러 발생
    // 러스트는 컴파일 시점에 number 변수의 타입이 무엇인지 정확히 알아야 함
    // let res = if condition {number} else {"Wrong Type"};
}
