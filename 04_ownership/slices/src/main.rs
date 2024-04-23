fn main() {
    // < 문자열 슬라이스>

   let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    let slice_string = String::from("hello");

    // [starting_index..ending_index]는 starting_index부터 시작해 ending_index 직전,
    //  즉 ending_index 에서 1을 뺀 위치까지 슬라이스를 생성한다는 의미
    //  인덱스 0부터 시작하는 경우, 앞의 값 생략 가능
    let slice = &slice_string[0..2];
    let slice = &slice_string[..2];

    let slice_len = slice_string.len();
    // 맨 마지막 바이트까지 포함하는 슬라이스의 뒤의 값 생략 가능
    let slice = &slice_string[2..slice_len];
    let slice = &slice_string[2..];

    // 앞 뒤 모두 생략할 경우, 전체 문자열이 슬라이스로 생성
    let slice = &slice_string[0..slice_len];
    let slice = &slice_string[..];

    // < 슬라이스로써의 문자열 리터럴 >

    let string_literal = "Hello world";
    // string_literal 은 바이너리의 특정 지점을 가리키는 슬라이스 &str 타입
    // &str 은 불변 참조자로 변경 불가

    let a = [1, 2, 3, 4, 5];
    let num_slice = &a[1..3];
    assert_eq!(num_slice, &[2, 3]);

}

fn first_word(s : &String) -> &str {
    // fn first_word(s: &str) -> &str { : String, &str 둘 다 매개변수로 사용 가능
    let bytes = s.as_bytes();
/*
    for (i, &item) in bytes.iter().enumerate(){
            if item == b' '{
                return i;
            }
    }
    s.len()

 */
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}