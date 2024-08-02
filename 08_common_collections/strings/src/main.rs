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


    // 2. 문자열 업데이트
    let mut s2 = String::from("hello, ");
    s2.push_str("Echichi");
    println!("{s2}");

    let mut s3 = String::from("hello, ");
    let mut s4 = String::from("world!");
    let s5 = s3 + &s4;  // s3는 여기로 이동되어 더 이상 사용 불가
    // println!("{s3}"); consider cloning the value if the performance cost is acceptable
    /*
        + 연산자의 시그니처
        fn add (self, s: &str) -> String{
        - String + String 은 안되지만 add 함수 내부에서 &String 인수가 &str 로 강제됨 (역참조 강제 변환)
        - s3의 소유권을 가져다가 s4의 내용물 복사본을 추가한 다음 결과물의 소육권을 s5로 반환
     */
    println!("{s5}");

    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
   // let s9 = s6 + "-" + s7 + "-" + s8;

    let s9 = format!("{s6}-{s7}-{s8}");
    println!("{s9}");


    // 3. 문자열 내부의 인덱싱
    let s10 = String::from("hello");
    let s11 = s10[0];   //  `String` cannot be indexed by `{integer}` 문자열 내부 인덱싱 불가

    let hello1 = String::from("Hola"); // 한 글자당 1바이트씩 len 은 4
    let hello2 = String::from("Здравствуйте"); // 한 글자당 2바이트씩 len 은 24
    // 바이트 단위로 인덱싱 되기 떄문에 바이트 경계를 무시하지 않기 위해서 직접 인덱싱 X
    // let answer = &hello2[0];
    // println!("{answer}")
    /*
        < 문자열 내부 인덱싱이 안되는 이유 >
        - 러스트에는 String을 분류하는 방법이 세 개가 있음 (바이트, 스칼라값, 문자소 클러스터)
        - 바이트 : 문자열을 UTF-8 로 인코딩 된 바이트 배열로 저장
        - 유니코드 스칼라 값 : 유니코드 코드 포인트 (발음 구별 기호 포함)
        - 문자소 클러스터 : 인간이 인식하는 문자 단위
     */

    // 4. 문자열 슬라이싱
    let hello = "Здравствуйте";
    let s = &hello[0..4];   // []와 범위를 사용하여 특정 바이트들이 담고 있는 문자열 슬라이스 생성
    // s = Зд 특정 바이트 배열이기 때문에 &hello[0..1]는 오류

    // 5. 문자열에 대한 반복을 위한 메서드
    // 5-1. 개별적인 유니코드 스칼라 값을 원할 때 .chars()
    for c in "Зд".chars() {
        println!("{c}");
    }

    // 5-2. 각 원시 바이트를 원할 때 .bytes()
    for b in "Зд".bytes(){
        println!("{b}");
    }

}
