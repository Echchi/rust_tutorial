/*
    3. 해시맵과 벡터를 이용하여 사용자가 회사 부서의 직원 이름을 추가할 수 있도록 하는
        텍스트 인터페이스를 만들어 보세요.
        예를 들어 ‘Add Sally to Engineering’이나 ‘Add Amir to Sales’ 같은 식으로요.
        그 후 사용자가 모든 사람에 대해 알파벳 순으로 정렬된 목록이나 부서별
        모든 사람에 대한 목록을 조회할 수 있도록 해보세요.
 */
use std::collections::HashMap;
fn main() {
    let mut department_map : HashMap<String, Vec<String>>= HashMap::new();
    loop{
        use std::io;
        let mut command = String::new();
        println!("Please write name and department.");
        println!("Add (name) to (Department).");


        io::stdin().read_line(&mut command).expect("Failed to read line");

        let command : String = match command.trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        add_member(&mut department_map, &command);

        for (department, names) in &department_map {
            println!("[ {} ]", department);
            for name in names {
                println!(" - {}", name);
            }
        }
    }

}

fn add_member(department_map : &mut HashMap<String, Vec<String>>, command : &String){
    let re = regex::Regex::new(r"Add (\w+) to (\w+)").unwrap();
    if let Some(caps) = re.captures(command){
        let name = caps.get(1).map_or("", |m| m.as_str());
        let department = caps.get(2).map_or("", |m| m.as_str());

        println!("Name: {}", &name);
        println!("Department: {}", &department);

        department_map.entry(department.to_string()).or_insert(Vec::new()).push(name.to_string());

    }else{
        println!("Invalid command format");
    }
}