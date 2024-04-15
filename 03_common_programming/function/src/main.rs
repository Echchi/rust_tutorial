fn main() {
    another_fn("echichi",26);

    let y = 0;   // 구문

    let y = {    // 표현식
        let x = 3;
        x + 1    // 세미콜론으로 끝나지 않으며 y 에 바인딩 됨
    };

    let z = five();
    println!("five is {z}");

    let res = plus_one(10);
    println!("10 + 1 = {res}");
}

fn another_fn(name:&str, age :i32){
    println!("🙋 My name is {name} and my age is {age}");
}

fn five() -> i32 {
    5
}

fn plus_one(num:i32) -> i32{
    num + 1    
    // num + 1 ; 로 세미콜론을 붙이면 i32가 반환된다고 되어있지만 아무것도 반환되지 않는다
    // 라는 에러 문구가 뜸
}
