/*
        2. 문자열을 피그 라틴 (pig Latin) 으로 변경해 보세요.
            각 단어의 첫 번째 자음은 단어의 끝으로 이동하고 ‘ay’를 붙이므로,
            first’는 ‘first-fay’가 됩니다. 모음으로 시작하는 단어는 대신 끝에 ‘hay’를 붙입니다.
            (‘apple’은 ‘apple-hay’가 됩니다.) UTF-8 인코딩에 대한 세부 사항을 명심하세요!
 */


fn main() {
    let word = "hello";
    let result = pig_latin(word);
    println!("pig_latin : {}", result);
}

fn pig_latin(word : &str) -> String {
    let vowels = ['a','e', 'i', 'o', 'u'];
    let first_char = word.chars().next().unwrap();

    if vowels.contains(&first_char){
        format!("{}-hay", word)
    }else{
        let rest = &word;
        format!("{}-{}ay", rest, first_char)
    }
}