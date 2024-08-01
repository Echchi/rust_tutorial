fn main() {
   let config_max = Some(3u8);
    /*
    match config_max {
        Some(max) => println!("The  maximum is configured to be {}", max),
        _ => (), // 보일러 플레이트
    }
     */

    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    }
}
