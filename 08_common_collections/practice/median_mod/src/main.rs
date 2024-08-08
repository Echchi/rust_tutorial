fn main() {
    /*
        1. 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 중간값 (median, 정렬했을 때 가장 가운데 위치한 값),
            그리고 최빈값 (mode, 가장 많이 발생한 값; 해시맵이 여기서 도움이 될 것입니다) 을 반환해 보세요.
            3, 1, 4, 1, 5, 9, 2
            8, 6, 7, 5, 3, 0, 9, 1
            5, 5, 5, 5, 5, 5, 5, 5
    */
    const list: [i32;7] = [3, 1, 4, 1, 5, 9, 2];
    let median = calculate_median(&list);
    let res = calculate_mod(&list);
    println!("median : {}, mode : {:?}", median, res);

}

fn calculate_median(numbers : &[i32]) -> f32{
    let mut v : Vec<i32> = numbers.to_vec();
    v.sort();

    let len = v.len();
    if len % 2 == 0 {
        (v[len/2 -1] + v[len/2]) as f32 / 2.0
    }else{
        v[len/2] as f32
    }
}

fn calculate_mod(numbers : &[i32]) -> Vec<i32>{
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for &i in numbers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("map : {:?}", &map);
    let mut mode = Vec::new();
    let mut max_count = 0;

    mode.push(numbers[0]);
    for (key, value) in map {
        if value > max_count {
            max_count = value;
            mode.clear();
            mode.push(key);
        } else if value == max_count {
            mode.push(key);
        }
    }

    println!("mode: {:?}", mode);
    mode
}



