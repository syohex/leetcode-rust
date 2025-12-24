fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let total: i32 = apple.into_iter().sum();
    let mut capacity = capacity;

    capacity.sort_unstable_by_key(|k| std::cmp::Reverse(*k));

    let mut ret = 0;
    let mut sum = 0;
    for c in capacity {
        ret += 1;
        sum += c;
        if sum >= total {
            return ret;
        }
    }

    ret
}

fn main() {
    let apple = vec![1, 3, 2];
    let capacity = vec![4, 3, 1, 5, 2];
    let ret = minimum_boxes(apple, capacity);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let apple = vec![1, 3, 2];
        let capacity = vec![4, 3, 1, 5, 2];
        let ret = minimum_boxes(apple, capacity);
        assert_eq!(ret, 2);
    }
    {
        let apple = vec![5, 5, 5];
        let capacity = vec![2, 4, 2, 7];
        let ret = minimum_boxes(apple, capacity);
        assert_eq!(ret, 4);
    }
}
