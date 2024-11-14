fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    fn can_distribute(count: i32, n: i32, quantities: &Vec<i32>) -> bool {
        let mut products = quantities[0];
        let mut store = 0usize;
        for _ in 0..n {
            if products <= count {
                store += 1;
                if store == quantities.len() {
                    return true;
                }

                products = quantities[store];
            } else {
                products -= count;
            }
        }

        false
    }

    let max_val = *quantities.iter().max().unwrap();

    let mut left = 1;
    let mut right = max_val;

    while left < right {
        let mid = left + (right - left) / 2;
        if can_distribute(mid, n, &quantities) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    let quantities = vec![15, 10, 10];
    let ret = minimized_maximum(7, quantities);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let quantities = vec![11, 6];
        let ret = minimized_maximum(6, quantities);
        assert_eq!(ret, 3);
    }
    {
        let quantities = vec![15, 10, 10];
        let ret = minimized_maximum(7, quantities);
        assert_eq!(ret, 5);
    }
}
