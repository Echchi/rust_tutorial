use std::net::IpAddr;

fn main() {
    let home : IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
}

pub struct Guess{
    value : i32,
}

/*
    Guess::new(50);
    잘못된 값으로 Guess 객체를 생성하려고 할 때 panic 을 발생시키고 프로그램 종료함
 */


impl Guess{
    pub fn new (value : i32) -> Guess {
        if(value < 1 || value > 100 ) {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    pub fn value(&self)->i32{
        self.value
    }
}