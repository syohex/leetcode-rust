fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
    let mut additional_rocks = additional_rocks;
    let mut diffs: Vec<i32> = capacity
        .into_iter()
        .zip(rocks)
        .map(|(a, b)| a - b)
        .collect();

    diffs.sort_unstable();

    let mut ret = 0;
    for diff in diffs {
        if diff == 0 {
            ret += 1;
        } else {
            if diff <= additional_rocks {
                ret += 1;
                additional_rocks -= diff;
            } else {
                break;
            }
        }
    }

    ret
}

fn main() {
    let capacity = vec![2, 3, 4, 5];
    let rocks = vec![1, 2, 4, 4];
    let ret = maximum_bags(capacity, rocks, 2);
    println!("ret={ret}");
}

#[test]
fn test_maximum_bags() {
    {
        let capacity = vec![2, 3, 4, 5];
        let rocks = vec![1, 2, 4, 4];
        let ret = maximum_bags(capacity, rocks, 2);
        assert_eq!(ret, 3);
    }
    {
        let capacity = vec![10, 2, 2];
        let rocks = vec![2, 2, 0];
        let ret = maximum_bags(capacity, rocks, 100);
        assert_eq!(ret, 3);
    }
}
