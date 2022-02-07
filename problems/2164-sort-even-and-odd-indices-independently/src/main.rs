fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut v_even = Vec::new();
    let mut v_odd = Vec::new();

    for i in 0..nums.len() {
        if i % 2 == 0 {
            v_even.push(nums[i]);
        } else {
            v_odd.push(nums[i]);
        }
    }

    v_even.sort();
    v_odd.sort_by(|a, b| b.cmp(a));

    let mut ret = Vec::new();
    for i in 0..v_even.len() {
        ret.push(v_even[i]);
        if let Some(v) = v_odd.get(i) {
            ret.push(*v);
        }
    }

    ret
}

fn main() {
    let nums = vec![4, 1, 2, 3];
    let ret = sort_even_odd(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_sort_even_odd() {
    {
        let nums = vec![4, 1, 2, 3];
        assert_eq!(sort_even_odd(nums), vec![2, 3, 4, 1]);
    }
    {
        let nums = vec![2, 1];
        assert_eq!(sort_even_odd(nums), vec![2, 1]);
    }
}
