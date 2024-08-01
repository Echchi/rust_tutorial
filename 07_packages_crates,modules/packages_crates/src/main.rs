/*

     < 모듈 시스템 > -  rust 코드 조직화 기능
     - 패키지 : 크레이트를 빌드, 테스트, 공유하는 카고 기능
     - 크레이트 : 라이브러리나 실행 가능한 모듈로 구성된 트리 구조 (일반적으로 라이브러리와 혼용)
     - 모듈과 use : 구조, 스코프를 제어하고 조직 세부 경로를 감추는데 사용
     - 경로 : 구조체, 함수, 모듈 등의 이름 지정


     1. cargo new packages_crates
     Created binary (application) `packages_crates` package // 패키지 생성
     2. ls packages_crates
     Cargo.lock  Cargo.toml  src  target    // packages_crates 라는 바이러니 크레이트만으로 구성

     src/bin 에는 각각의 바이너리 크레이트 저장

*/

fn main() {
    println!("Hello, world!");
}
