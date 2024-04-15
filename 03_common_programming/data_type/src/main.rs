fn main() {
    // 스칼라 타입 : 정수, 부동 소수점 숫자, 부울린, 문자
    
    // 부동 소수점
    // 기본 타입 f64
    // f32 타입은 1배 수 정밀도 (single-precision) 인 부동 소수점이고, f64는 2배 수 정밀도 (double-precision)
    let x = 2.0; // f64

    let y : f32 = 3.0; // f32

    // 덧셈
    let sum = 1 + 1;
    
    // 뺄셈
    let difference = 80.5 - 2.5;

    // 곱셈
    let mul = 2 * 40;

    // 나눗셈
    let devide = 56.7 / 32.2;
    let truncated = -5 / 3;

    // 나머지
    let remain = 42 % 5;

    // 부울린 타입 (1바이트)
    let t = true; 

    let f : bool = false; // 명시적 타입 어노테이션

    // 문자 타입
    // 문자열은 큰따옴표, char 타입(4바이트) 은 작은 따옴표
    let c = 'y';
    let z: char = 'Z'; // 명시적 타입 어노테이션
    let ferris = '🦀';

    // 복합 타입 (튜플, 배열)
    // 튜플
    // 고정된 길이를 가지며  튜플 내의 타입들은 서로 달라도 됨
    let tup = (100, 200, 300);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let tup2 = (400, 500, 600);

    let four_hundred = x.0;

    let five_hundred = x.1;

    let six_hundred = x.2;
    
    // 배열
    // 고정된 길이를 가지며 배열 내의 요소들의 타입은 모두 같아야함
    let arr = [1, 2, 3, 4, 5];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let jan = months[0];
    let nov = months[10];

    let four_three = [3; 4];    // let four_three = [3, 3, 3, 3]


}
