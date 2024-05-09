/*
< 열거형 정의 >
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr{
    kind : IpAddrKind,
    address : String
}

 */

enum IpAddr {   // 각 열거형 배리언트에 데이터를 직접 넣는 방식
    V4(u8, u8, u8, u8),
    V6(String),
}

// < Option 열거형 >
enum Option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move  {x : i32 , y : i32},
    Write (String),
    ChangeColor (i32, i32, i32)
}

impl Message {  // struct 에 impl을 사용해서 메서드를 정의한 것처럼 열거형에도 정의 가능
    fn call (name : &str){
        println!("Hello my name is {}", name)
    }
}


fn main() {
    /*
    let home = IpAddr{
        kind : IpAddrKind::V4,
        address : String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind : IpAddrKind::V6,
        address : String::from("::1"),
    };

     */
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    Message::call("Echichi");

    let x : i8 = 5;
    let y : Option<i8> = Option::Some(5);
    let z : Option<i8> = Some(5);

    println!("Is it the Some value? {}", y.is_some())

    // let sum = x + y;
    /*
        T에 대한 연산을 수행하기 전에 Option<T>를 T로 변환해야함

        error[E0277]: cannot add `Option<i8>` to `i8`
          --> src/main.rs:59:17
           |
        59 |     let sum = x + y;
           |                 ^ no implementation for `i8 + Option<i8>`
           |
           = help: the trait `Add<Option<i8>>` is not implemented for `i8`
           = help: the following other types implement trait `Add<Rhs>`:
             <i8 as Add>
             <i8 as Add<&i8>>
             <&'a i8 as Add<i8>>
             <&i8 as Add<&i8>>
    */


}
