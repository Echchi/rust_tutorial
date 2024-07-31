mod front_of_house;

/*
    모듈 트리 구조
    crate
     └── front_of_house
         ├── hosting
         │   ├── add_to_waitlist
         │   └── seat_at_table
         └── serving
             ├── take_order
             ├── serve_order
             └── take_payment

 */


fn deliver_order(){}

mod back_of_house{
    pub struct Breakfast{
        pub toast : String,
        seasonal_fruit:String
    }

    impl Breakfast {
        pub fn summer (toast:&str) -> Breakfast{
            Breakfast{
                toast : String :: from(toast),
                seasonal_fruit : String :: from("peaches")
            }
        }

    }

    pub enum Appetizer{
        Soup,
        Salad,
    }



    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){}
}


pub fn eat_at_restaurant(){
    // 절대 경로
    // crate::front_of_house::hosting::add_to_waitlist();
    // 상대 경로
    // front_of_house::hosting::add_to_waitlist();

    // 호밀 토스트를 곁들인 여름철 조식 주문
    let mut meal = back_of_house::Breakfast::summer("호밀");
    // 밀 토스트로 바꾸기
    meal.toast = String::from("밀");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    // 제공되는 계절 과일은 조회나 수정 허용 X

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

}

mod customer{
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }
}


// use 키워드를 사용하여 단축경로 사용 가능 (심볼릭링크와 유사)
// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting; // 다시 내보내기
/*
    use crate::front_of_house::hosting::add_to_waitlist;
    라고 구체적으로 선언하지 않은 이유!
    - 함수 호출 시 부모 모듈을 특정하면 전체 경로를 반복하는 것을 최소화
    - 로컬에 정의되어있지 않음을 명시
    - 반명 구조체나 열거형 등의 타 아이템을 가져올 때는 전체 경로로 작성
    - 이름이 중복될 때는 as 키워드로 새로운 이름 제공
 */
/*
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io}; // 중첩 경로 사용 가능

use std::io;
use std::to::Write;

use std::io::{self, Write}; // 중첩 경로 사용 가능

use std::collections::*; // 글롭 연산자로 경로 안에 정의된 모든 공개 아이템 사용 가능

 */