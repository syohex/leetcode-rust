fn divide_players(skill: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let mut sum = 0i64;
    let mut h = HashMap::new();

    for &n in &skill {
        sum += n as i64;
        *h.entry(n as i64).or_insert(0i64) += 1i64;
    }

    let mut ret = 0i64;
    let target = sum / (skill.len() as i64 / 2i64);
    for n in skill {
        let n = n as i64;
        let diff = target - n;
        if let Some(v) = h.get_mut(&diff) {
            if *v > 0 {
                ret += n * diff;
                *v -= 1;
            } else {
                return -1;
            }
        } else {
            return -1;
        }
    }

    ret / 2
}

fn main() {
    let skill = vec![3, 2, 5, 1, 3, 4];
    let ret = divide_players(skill);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let skill = vec![3, 2, 5, 1, 3, 4];
        let ret = divide_players(skill);
        assert_eq!(ret, 22);
    }
    {
        let skill = vec![3, 4];
        let ret = divide_players(skill);
        assert_eq!(ret, 12);
    }
    {
        let skill = vec![1, 1, 2, 3];
        let ret = divide_players(skill);
        assert_eq!(ret, -1);
    }
    {
        let skill = vec![4, 2, 4, 3];
        let ret = divide_players(skill);
        assert_eq!(ret, -1);
    }
}
