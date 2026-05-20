fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut freq = vec![0; 51];
    let mut count = 0;
    let mut ret = vec![];
    for (num1, num2) in a.into_iter().zip(b) {
        freq[num1 as usize] += 1;
        if freq[num1 as usize] == 2 {
            count += 1;
        }

        freq[num2 as usize] += 1;
        if freq[num2 as usize] == 2 {
            count += 1;
        }

        ret.push(count);
    }

    ret
}

fn main() {
    let ret = find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
        vec![0, 2, 3, 4]
    );
    assert_eq!(
        find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
        vec![0, 1, 3]
    );
}
