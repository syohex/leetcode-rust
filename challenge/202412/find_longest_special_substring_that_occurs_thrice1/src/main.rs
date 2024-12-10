fn maximum_length(s: String) -> i32 {
    use std::collections::HashMap;

    let cs :Vec<_> = s.chars().collect();
    let mut h: HashMap<Vec<char>, i32> = HashMap::new();

    for i in 0..cs.len() {
        let mut tmp = vec![];
        for j in i..cs.len() {
            if tmp.is_empty() || *tmp.last().unwrap() == cs[j] {
                tmp.push(cs[j]);
                *h.entry(tmp.clone()).or_insert(0) += 1;
            } else {
                break;
            }
        }
    }

    let mut ret = -1;
    for (k, v) in &h {
       if *v >= 3 && k.len() as i32 >= ret {
           ret = k.len() as i32;
       }
    }

    ret
}

fn main() {
    let ret = maximum_length("aaaa".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_length("aaaa".to_string()), 2);
    assert_eq!(maximum_length("abcdef".to_string()), -1);
    assert_eq!(maximum_length("abcaba".to_string()), 1);
}
