fn total_fruit(fruits: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let len = fruits.len();
    let mut h = HashMap::new();
    let mut left = 0;

    let mut ret = 0;
    for i in 0..len {
        *h.entry(fruits[i]).or_insert(0) += 1;

        loop {
            if h.len() <= 2 {
                break;
            }

            let mut should_remove = false;
            if let Some(v) = h.get_mut(&fruits[left]) {
                *v -= 1;
                if *v == 0 {
                    should_remove = true;
                }
            }
            if should_remove {
                h.remove(&fruits[left]);
            }

            left += 1;
        }

        ret = std::cmp::max(ret, i - left + 1);
    }

    ret as i32
}

fn main() {
    let fruits = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    let ret = total_fruit(fruits);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let fruits = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
        let ret = total_fruit(fruits);
        assert_eq!(ret, 5);
    }
    {
        let fruits = vec![1, 2, 1];
        let ret = total_fruit(fruits);
        assert_eq!(ret, 3);
    }
    {
        let fruits = vec![0, 1, 2, 2];
        let ret = total_fruit(fruits);
        assert_eq!(ret, 3);
    }
    {
        let fruits = vec![1, 2, 3, 2, 2];
        let ret = total_fruit(fruits);
        assert_eq!(ret, 4);
    }
}
