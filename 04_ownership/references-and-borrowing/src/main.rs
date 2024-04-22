fn main() { let tmp_string = String::from("I'll be back");
   let mut mut_tmp_string = String::from("I'll be back");   // 가변 참조자 생성

    let len = caclulate_length(&tmp_string);

    println!("The length of '{}' is {}.", tmp_string, len);

    // < 가변 참조자 >

    // change(&tmp_string)
    change(&mut mut_tmp_string); // 가변 참조자 전달
    let can_i_use_mut_tmp = &mut mut_tmp_string;
    let can_i_use_mut_tmp_twice = &mut mut_tmp_string;

    // println!("No, I can't use {} and {}", can_i_use_mut_tmp, can_i_use_mut_tmp_twice);

    /*
        error[E0499]: cannot borrow `mut_tmp_string` as mutable more than once at a time
          --> src/main.rs:12:35
           |
        11 |     let can_i_use_mut_tmp = &mut mut_tmp_string;
           |                             ------------------- first mutable borrow occurs here
        12 |     let can_i_use_mut_tmp_twice = &mut mut_tmp_string;
           |                                   ^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
        13 |
        14 |     println!("No, I can't use {} and {}", can_i_use_mut_tmp, can_i_use_mut_tmp_twice);
           |                                           ----------------- first borrow later used here

    */

    let mut always = String::from("always");
    {
        let changer = &mut always;

    }   // changer 이 스코프 밖으로 벗어나기 때문에 아무 문제 없이 새 참조자 생성 가능
    let real_changer = &mut always;


    let mut is_problem = String::from("no");

    let not_mut_string = &is_problem;    // 불변 참조자 문제없음
    let not_mut_string_2 = &is_problem;
    println!("{} and {}", not_mut_string, not_mut_string_2);

    let mut_string = &mut is_problem;
    println!("{}", mut_string);

    /*
        let not_mut_string = &is_problem;       // 불변 참조자 문제없음
        let not_mut_string_2 = &is_problem;     // 불변 참조자 문제없음

        let mut_string = &mut is_problem;       // 큰 문제


        println!("{}, {}, and {}", not_mut_string, not_mut_string_2, mut_string);
        어떤 값에 대한 불변 참조자가 있는 동안 같은 값의 가변 참조자를 만드는 것 불가능
        error[E0502]: cannot borrow `is_problem` as mutable because it is also borrowed as immutable
      --> src/main.rs:43:22
       |
    41 |     let not_mut_string = &is_problem;    // 불변 참조자 문제없음
       |                          ----------- immutable borrow occurs here
    42 |     let not_mut_string_2 = &is_problem;    // 불변 참조자 문제없음
    43 |     let mut_string = &mut is_problem;    // 가변 참조자 큰 문제
       |                      ^^^^^^^^^^^^^^^ mutable borrow occurs here
    44 |
    45 |     println!("{}, {}, and {}", not_mut_string, not_mut_string_2, mut_string);
       |                                -------------- immutable borrow later used here
    */

    // < 댕글링 참조 >

    let reference_to_nothing = no_dangle();
}
fn caclulate_length (s : &String) -> usize{
    s.len()
}   // s가 스코프 밖으로 벗어나지만 "소유"가 아닌 "참조"로 버려지지않음

/*
fn change(some_string :&String){
     some_string.push_str(", haha");

        변수가 기본적으로 불변성을 지니듯, 참조자도 마찬가지로 참조하는 것을 수정할 수 없음

        error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
          --> src/main.rs:15:5
           |
        15 |     some_string.push_str(", haha");
           |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

}
 */

fn change(some_string :&mut String){    // 가변 참조자를 전달받도록 수정
     some_string.push_str(", haha");
    println!("{}", some_string);
}

/*
fn dangle() -> &String{     // String 참조자 반환
                            //  this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    let s = String::from("dangle dangle");  // s는 새로운 String

    &s  // String s의 참조자 반환
}   // s는 스코프 밖으로 벗어나고 버려짐 -> drop 함수 발동 -> 메모리 해제
 */

fn no_dangle() -> String {
    let s = String::from("no dangle no dangle");

    s
}