fn main() {
    // < 변수의 스코프 >

    {   // 스코프 시작 s 유효
        let s = "hello";
    }   // 스코프 종료 x -> drop 함수 호출 -> 메모리 해제 -> 유효 x

    // -------------------------------------------------------

    // < String 타입 >

    let mut input = String::from("hello");
    input.push_str(", echichi!");
    println!("{}",input);

    // -------------------------------------------------------

    // < 메모리와 할당 >

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, echichi!", s2);
    // println!("{}, echichi!", s2);
    /*
        유효하지 않은 참조자 사용 감지 에러 -> s1 무효화
        For more information about this error, try `rustc --explain E0382`.
        warning: `ownership` (bin "ownership") generated 2 warnings
        error: could not compile `ownership` (bin "ownership") due to 1 previous error;
        2 warnings emitted

         - 기존의 변수를 무효화하기 때문에 얕은 복사가 아닌 "이동"
            -> s1 이 s2로 "이동" 되었다
         - 러스트는 절대 자동으로 깊은 복사 x
    */

    // -------------------------------------------------------

    // < 변수와 데이터 간 상호작용 방식: 클론 >

    let clone_s2 = s2.clone();
    println!("{}, echichi!", clone_s2);

    // clone :깊은 복사
    // 해당 위치에서 무언가 다른 일이 수행될 것을 알려주는 시각적인 표시

    // -------------------------------------------------------

    // < 스택에만 저장되는 데이터: 복사 >

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // 정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장
    // -> copy 트레이트가 구현 되어있기 때문 (불변성)
    /*
        - 모든 정수형 타입
        - true, false 값을 갖는 논리 자료형 : boolean
        - 문자 타입 char
        - copy 가능한 타입으로만 구성된 튜플
          ex) (i32, i32) 가능, (i32, String) String 때문에 불가능
     */

    // -------------------------------------------------------

    // < 소유권과 함수 >

    let heap_var = String::from("hello");   // heap_var 이 스코프 안에 등장
    takes_ownership(heap_var);              // heap_var 값이 함수로 이동

    // println!("{}", heap_var);            // heap_var 값이 이동한 후이기 떄문에 유효 X
    // note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary

    let stack_var = 5;                 // stack_var 이 스코프 안에 등장
    makes_copy(stack_var);                  // stack_var 값이 함수로 이동

    println!("{}", stack_var);              // i32는 copy 이므로 사용 가능

    // -------------------------------------------------------

    // < 반환 값과 스코프 >

    let receiver = gives_ownership();       // gives_ownership 이 자신의 반환값을 receiver로 이동

    let gift = String::from("🎁");          // gift가 스코프 안에 등장

    let final_receiver = takes_and_gives_back(gift);    // gift가 takes_and_gives_back로 이동되는데
}

fn takes_ownership(some_string: String) {   // some_string이 스코프 안에 등장
    println!("takes_ownership fn takes '{}' ownership ", {some_string});
}   // some_string 스코프 밖으로 벗어남 -> drop 함수 호출 -> 메모리 해제

fn makes_copy(some_integer : i32){  // some_integer 가 스코프 안에 등장
    println!("makes_copy fn receives a copy of '{}', not ownership", some_integer);
}   // some_integer 가 스코프 밖으로 벗어남

fn gives_ownership() -> String {    // gives_ownership은 자신의 반환값을 자신의 호출자 함수로 이동시킬 예정
    let some_string = String::from("✉️");   // some_string 이 스코프 안에 등장
    some_string     // some_string 이 반환되고 호출자 함수 쪽으로 이동
    
}