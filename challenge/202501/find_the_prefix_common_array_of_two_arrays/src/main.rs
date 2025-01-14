fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut freq = vec![0; a.len()];
    let mut ret = vec![];
    let mut commons = 0;
    for (m, n) in a.into_iter().zip(b.into_iter()) {
        let (m, n) = (m as usize, n as usize);
        freq[m - 1] += 1;
        if freq[m - 1] == 2 {
            commons += 1;
        }

        freq[n - 1] += 1;
        if freq[n - 1] == 2 {
            commons += 1;
        }

        ret.push(commons);
    }

    ret
}

fn main() {
    let a = vec![1, 3, 2, 4];
    let b = vec![3, 1, 2, 4];
    let ret = find_the_prefix_common_array(a, b);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let a = vec![1, 3, 2, 4];
        let b = vec![3, 1, 2, 4];
        let expected = vec![0, 2, 3, 4];
        let ret = find_the_prefix_common_array(a, b);
        assert_eq!(ret, expected);
    }
    {
        let a = vec![2, 3, 1];
        let b = vec![3, 1, 2];
        let expected = vec![0, 1, 3];
        let ret = find_the_prefix_common_array(a, b);
        assert_eq!(ret, expected);
    }
}
