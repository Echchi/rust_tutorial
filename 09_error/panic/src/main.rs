/*
    panic! : 실패 메세지를 출력하고 되감고 스택을 청소하고 종료함
    unwinding (되감기) : 패닉을 발생시킨 각 함수로부터 스택을 거꾸로 훑어가면서 데이털르 청소함
    aborting (그만두기) : 정리 작업 없이 즉각 종료
 */

fn main() {
    // panic!("crash 💔 and burn 🔥");

    let v = vec![1, 2, 3];
    // v[99];
}

