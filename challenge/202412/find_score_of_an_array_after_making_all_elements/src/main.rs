fn find_score(nums: Vec<i32>) -> i64 {
    let mut v: Vec<_> = nums.into_iter().enumerate().map(|(i, n)| (n, i)).collect();
    v.sort_unstable();

    let len = v.len();
    v.into_iter()
        .fold((0i64, vec![false; len]), |(acc, mut checked), (n, i)| {
            if checked[i] {
                (acc, checked)
            } else {
                checked[i] = true;
                if i >= 1 {
                    checked[i - 1] = true;
                }
                if i + 1 < checked.len() {
                    checked[i + 1] = true;
                }
                (acc + n as i64, checked)
            }
        })
        .0
}

fn main() {
    let nums = vec![2, 1, 3, 4, 5, 2];
    let ret = find_score(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 3, 4, 5, 2];
        let ret = find_score(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![2, 3, 5, 1, 3, 2];
        let ret = find_score(nums);
        assert_eq!(ret, 5);
    }
}
