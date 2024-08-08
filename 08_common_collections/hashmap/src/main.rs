/*
    해시맵 (HashMap<K, V>)

 */
fn main() {
    // 1. 해시맵 값 접근
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get 메서드는 Option<&V>를 반환
    // copied 를 호출하여 Option<i32>를 얻어온 다음 unwrap_or 을 써서 scores 가
    // 해당 키에 대한 아이템을 가지고 있지 않을 경우 score에 0을 설정하도록 처리
    println!("{score}");

    // 2. 해시맵 반복 작업
    for (key, value) in &scores {
        println!("{key} : {value}")
    }

    // 3. 해시맵 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name, field_value은 이 시점부터 유효하지 않음!
    // i32 처럼 Copy 트레이트가 구현된 타입은 해시맵 안으로 복사되지만
    // String 처럼 소유권이 있는 값의 경우 소유권이 해시맵으로 넘어감

    // 4. 해시맵 업데이트
    // 4-1. 값 덮어쓰기
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 100);

    println!("{:?}", scores);

    // 4-2. 키가 없을 때 키와 값 추가
    // 옐로, 블루 팀에 대한 키에 대한 값이 있는지 검사하고 없다면 값 추가
    scores.entry(String::from("Yellow")).or_insert(150);
    scores.entry(String::from("Blue")).or_insert(150);

    println!("{:?}", scores);

    // 4-3. 예전 값에 기초하여 값 업데이트
    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace(){
        let count = map2.entry(word).or_insert(0);  // text의 값을 공백문자로 나눈 서브 슬라이스에 대한 반복자 반환
        *count += 1;    // 애프터리스크를 사용하여 count 역참조
    }

    println!("{:?}", map2);

}
