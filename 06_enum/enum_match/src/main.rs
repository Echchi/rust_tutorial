#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)    // 값 바인딩
}


fn value_in_cents(coin: Coin)->u8{
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Option<T> 를 이용하는 매칭
fn plus_one(x : Option<i32>) -> Option<i32>{
    match x {
        None => None,   // 생략시 에러 발생  pattern `None` not covered
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_car(){
    println!("🚗");

}
fn remove_fancy_car(){
    println!("🚗 X");

}
fn move_player(num_spaces : u8){
    for _ in 0 .. num_spaces {
     println!("🏃‍♀️")
    }
}

fn reroll(){

}

fn main() {
   value_in_cents(Coin::Quarter(UsState::NewYork));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six! {:?}", six);
    println!("none! {:?}", none);

    // 포괄 패턴과 _ 자리 표시자
    let dice_roll = 9;
    match dice_roll{
        3 => add_fancy_car(),
        7 => remove_fancy_car(),
        // other => move_player(other) // 포괄 패턴 : 모든 값을 나열하지 않았음에도 컴파일
        // _ => reroll()   // _ : 포괄 패턴이 필요한데 그 포괄 패턴의 값을 사용할 필요는 없는 경우에 쓸 수 있는 패턴
        _ => (),    // 유닛 값 : 어떠한 값도 사용하지 않을 것이며, 어떠한 코드도 실행하지 않기를 원한다고 명시적으로 알려준 것
    }
}
