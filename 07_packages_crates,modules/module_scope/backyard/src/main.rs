/*
    < 모듈 차트 시트 >
    컴파일러 작동 순서
    1. 크레이트 루트파일 보기 👀
     - 라이브러리 크레이트 (src/lib.rs)
     - 바이너리 크레이트 (src/main.rs)

    2. 모듈 코드가 있는지 살펴보기 👀
     (mod garden; 이라는 코드로 garden 모듈 선언)
     - mod garden 뒤에 세미콜론 대신 중괄호 써서 안쪽에 코드를 적은 인라인
     - src/garden.rs 파일 안
     - src/garden/mod.rs 파일 안

    3. 서브 모듈이 있는지 살펴보기 👀
     (src/garden.rs 안에 mde vegetables; 서브 모듈 선언)
     - mod vegetables 뒤에 세미콜론 대신 중괄호를 써서 안쪽에 코드를 적은 인라인
     - src/garden/vegetables.rs 파일 안
     - src/garden/vegetables/mod.rs 파일 안

    4. 모듈 내 코드로의 경로 살펴보기 👀
     - 모듈이 크레이트의 일부로 구성되면 공개 규칙이 허용하는 한도 내에서 해당 코드의 경로를
       사용하여 동일한 크레이트 어디서든 코드 참조 가능
       (garden vegetables 모듈 안에 있는 Asparagus 는 crate::garden::vegetables::Asparagus로 참조 가능)

    5. 비공개 vs 공개
     - 모듈 내의 코드는 기본적으로 부모 모듈에게 비공개
     - 공개로 만들려면 mod 대신 pub mod로 선언

    6. use 키워드
     - 어떤 스코프 내에서 긴 경로의 반복을 줄이기 위한 어떤 아이템으로의 단축 경로 생성
     use crate::garden::vegetables::Asparagus 로 단축경로를 만들면 이후부터 스코프에서 Asparagus만 작성 가능

 */
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
