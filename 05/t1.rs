use std::fs;
use std::collections::HashMap;

fn count_correct(rules_hash_map: HashMap<&str, Vec<&str>>, page_numbers: &str) -> bool{
        let tmp = page_numbers.split(",").collect::<Vec<&str>>();
        for i in 0..tmp.len()-1 {
            for j in i+1..tmp.len() {
                match rules_hash_map.get(&tmp[i]) {
                    Some(vec) => {
                        if !vec.contains(&tmp[j]) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    

    return true;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let rules = parts[0];
    let page_numbers = parts[1];
    let mut rules_hash_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for rule in rules.split("\n") {
        let tmp = rule.split("|").collect::<Vec<&str>>();
        rules_hash_map.entry(tmp[0]).and_modify(|vec| vec.push(tmp[1])).or_insert(vec![tmp[1]]);
    }

    let mut count = 0;
    let mut answer = 0;
    for page_numbers_line in page_numbers.split("\n").collect::<Vec<&str>>() {
        if page_numbers_line.len() == 0 {
            break;
        }
        let ok = count_correct(rules_hash_map.clone(), page_numbers_line);
        if ok {
            let tmp = page_numbers_line.split(",").collect::<Vec<&str>>();
            answer += tmp[tmp.len()/2].parse::<i32>().expect("parsing error");
            count += 1;
        }
    }

    println!("true_count: {count}");
    println!("answer: {answer}");
}
