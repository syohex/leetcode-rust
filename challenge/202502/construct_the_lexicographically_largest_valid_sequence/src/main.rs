fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    fn f(pos: usize, n: usize, acc: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
        if pos == acc.len() {
            return used.iter().all(|&n| n);
        }

        if acc[pos] != 0 {
            return f(pos + 1, n, acc, used);
        }

        for i in (1..=n).rev() {
            if used[i] {
                continue;
            }

            let another_index = if i != 1 { pos + i } else { pos };
            if another_index >= acc.len() || acc[another_index] != 0 {
                continue;
            }

            acc[pos] = i as i32;
            acc[another_index] = i as i32;

            used[i] = true;
            if f(pos + 1, n, acc, used) {
                dbg!(&acc);
                return true;
            }

            used[i] = false;
            acc[pos] = 0;
            acc[another_index] = 0;
        }

        false
    }

    let n = n as usize;
    let mut used = vec![false; n + 1];
    used[0] = true;

    let mut acc = vec![0; 2 * n - 1];
    f(0, n, &mut acc, &mut used);
    acc
}

fn main() {
    let ret = construct_distanced_sequence(5);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let n = 3;
        let expected = vec![3, 1, 2, 3, 2];
        let ret = construct_distanced_sequence(n);
        assert_eq!(ret, expected);
    }
    {
        let n = 5;
        let expected = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];
        let ret = construct_distanced_sequence(n);
        assert_eq!(ret, expected);
    }
}
