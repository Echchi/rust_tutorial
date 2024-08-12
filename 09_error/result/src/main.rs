use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;


fn main() {
    enum Result<T,E>{
        Ok(T),
        Err(E),
    }
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file : {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file : {:?}", other_error);
    //         }
    //     }
    // };

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             panic!("Problem creating the file : {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file : {:?}",error);
    //     }
    // });

    // expect 를 이용하면 에러 메세지 제공 가능
    let greeting_file = File::open("hello.txt").expect("where are you hello.txt??");

    // let greeting_file = File::open("hello.txt")?;   // ? 는 Result, Option 혹은 FromResidual을 구현한 타입을 반환하는 함수에서만 사용가능



}

/*

fn read_username_from_file()->Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Error(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? 연산자를 사용하여 보일러 플레이트 제거 가능
fn read_username_from_file()->Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);
}



// ? 연산자 뒤에 메서드 호출을 연결 가능
fn read_username_from_file () -> Result<String, io::Error>{
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

 */

// fs::read_to_string를 사용하여 더 짧게 가능 (파일을 열고 새 String 을 생성하고 파일 내용을 읽고 반환하는 함수)
fn read_username_from_file()->Result<String, E>{
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str)-> Option<char>{
    text.lines().next()?.chars().last()
}