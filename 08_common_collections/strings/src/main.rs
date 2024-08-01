/*
    문자열 : 가변적이며 소유권을 갖고있고 UTF-8로 인코딩된 문자열 타입
    - &str 은 문자열 슬라이스로 어딘가에 저장된 문자열 데이터의 참조자
 */

fn main() {
    // 1. 새로운 문자열 생성
    // 방법 1)
    let mut s1 = String::new();
    let data = "초깃값";
    let s1 = data.to_string();

    // 방법 2)
    // let s1 = "초깃값".to_string();

    // 방법 3)
    // let s1 = String::from("초깃값");

}
