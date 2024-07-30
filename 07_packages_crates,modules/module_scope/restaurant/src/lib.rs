mod front_of_house{
    pub mod hosting{
       pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

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

}