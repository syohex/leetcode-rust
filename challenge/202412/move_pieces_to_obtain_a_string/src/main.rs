fn can_change(start: String, target: String) -> bool {
    use std::collections::VecDeque;

    let len = start.len();
    let cs1: Vec<char> = start.chars().collect();
    let cs2: Vec<char> = target.chars().collect();
    let mut q1 = VecDeque::new();
    let mut q2 = VecDeque::new();

    for i in 0..len {
        if cs1[i] != '_' {
            q1.push_back((i, cs1[i]));
        }
        if cs2[i] != '_' {
            q2.push_back((i, cs2[i]));
        }
    }

    if q1.len() != q2.len() {
        return false;
    }

    while !q1.is_empty() {
        let (pos1, c1) = q1.pop_front().unwrap();
        let (pos2, c2) = q2.pop_front().unwrap();

        if c1 != c2 {
            return false;
        }

        if c1 == 'L' && pos1 < pos2 {
            return false;
        }

        if c1 == 'R' && pos1 > pos2 {
            return false;
        }
    }

    true
}

fn main() {
    let ret = can_change("R_L_".to_string(), "__LR".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_change("_L__R__R_".to_string(), "L______RR".to_string()));
    assert!(!can_change("R_L_".to_string(), "__LR".to_string()));
    assert!(!can_change("_R".to_string(), "R_".to_string()));
}
