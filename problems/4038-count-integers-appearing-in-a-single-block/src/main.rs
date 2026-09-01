fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut positions = vec![vec![]; 101];

    for (i, n) in nums.into_iter().enumerate() {
        positions[n as usize].push(i);
    }

    let mut ret = 0;
    for v in positions {
        match v.len() {
            0 => (),
            1 => ret += 1,
            n => {
                let mut ok = true;
                for i in 1..n {
                    if v[i] - v[i - 1] != 1 {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    ret += 1;
                }
            }
        }
    }

    ret
}

fn main() {
    let ret = count_special_integers(vec![3, 3, 1, 2, 2, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_special_integers(vec![22]), 1);
    assert_eq!(count_special_integers(vec![22, 11]), 2);
    assert_eq!(count_special_integers(vec![1, 2, 2, 1]), 1);
    assert_eq!(count_special_integers(vec![3, 3, 1, 2, 2, 1]), 2);
}
