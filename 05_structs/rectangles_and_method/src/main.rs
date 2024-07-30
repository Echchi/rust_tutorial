#[derive(Debug)]
struct Reactangle{
    width : u32,
    height : u32,
}

impl Reactangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Reactangle{
    fn can_hold(&self, other : &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
   // let scale = 2;
   let rect1 = Reactangle{
       width : 30,
       // width: dbg!(30 * scale), // 타입에 대한 보기 좋은 Debug 포맷 제공
       height : 50,
   };

    let rect2 = Reactangle{
        width : 30,
        height : 20,
    };

    // println!("👀 react1 is {} 👀", rect1);
    // the trait `std::fmt::Display` is not implemented for `Reactangle
    // println!("react1 is {:#?}", rect1);
    // the trait `Debug` is not implemented for `Reactangle` -> #[derive(Debug)] 추가
    // (Rectangle 인스턴스를 디버그 출력 형식으로 사용하기 위해, 속성을 추가하여 Debug 트레이트 파생 (derive) )
    // println!("👀 The area of the rectangle is {} square pixels 👀", area(&rect1));

    // 리팩토링
    println!("👀 The area of the rectangle is {} square pixels 👀", rect1.area());
    println!("👀 Can rect1 hold rect2? {} 👀", rect1.can_hold(&rect2));
}

// fn area(reactangle: &Reactangle) -> u32{
//     reactangle.width * reactangle.height
// }
