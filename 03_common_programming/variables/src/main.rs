fn main() {
    let mut x = 10;
    // mut : 변수를 가변으로 만듬
    println!("The value of x is: {x}");
    x = 5;
    println!("The value of x is: {x}");

    
    let y = 2;

    let y = y + 1;

    // shadowing : 첫 번째 변수가 두 번째 변수에 의해 가려졌다 (y 값이 바뀜)
    // let 을 사용하면 값을 변형하면서 변형이 완료된 후에는 불변으로 유지
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

}
