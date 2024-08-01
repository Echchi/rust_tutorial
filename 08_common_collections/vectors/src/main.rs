/*
    Vec<T> 벡터 : 벡터를 사용하면 메모리에서 모든 값을 서로 이웃하도록 배치하는
                  단일 데이터 구조에 하나 이상의 값 저장 가능
 */

fn main() {
    // 1. 벡터 생성
    let v1 : Vec<i32> = Vec::new();  // 새 벡터 생성
    let v1 = vec![1, 2, 3];

    // 2. 벡터 업데이트
    let mut v2 = Vec::new();
    // 타입 명시를 안하는 이유는 집어 넣은 숫자가 모두 i32 타입으로 v2의 타입 추론
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(6);

    // 3. 벡터 읽기
    let v3 = vec!["🍕","🍔","🌭","🍟"];

    let third: &str = &v3[2];
    println!("세번째 음식은 {third}");

    let third : Option<&&str> = v3.get(2);
    match third {
        Some(third) => println!("get을 이용한 세번째 음식은 {third}"),
        None => println!("세번쨰 음식이 없어요"),
    }

    let v4 = vec!["🍕","🍔","🌭","🍟"];

    // let does_not_exist = &v4[100];
    // let does_not_exist2 = v4.get(100);

    let mut v5 = vec!["🍕","🍔","🌭","🍟"];

    let first = &v5[0];

    v5.push("🦐");

    // println!("첫번쨰 음식은 : {first}")
    // 요소 추가 시 힙 메모리 재할당으로 기존 참조를 무효화시켜 참조 불가

    // 4. 벡터 값 반복
    let v6 = vec!["❤️", "🩷", "🧡"];
    for i in 0..3 {
        println!("{}: {}", i, &v6[i])
    }

    let mut v7 = vec![1, 10, 100];
    for i in &mut v7 {
        *i += 50;
        println!("{i}");
    }

    // 열거형을 통해 여러 타입 저장
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Echichi")),
        SpreadsheetCell::Float(8.01),
    ];

}
