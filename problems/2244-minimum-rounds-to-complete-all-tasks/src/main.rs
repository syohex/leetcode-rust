fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for task in tasks {
        *h.entry(task).or_insert(0) += 1;
    }

    dbg!(&h);
    let mut ret = 0;
    for (_, v) in h.iter() {
        let mut v = *v;

        while v >= 2 && v % 3 != 0 {
            v -= 2;
            ret += 1;
        }
        
        if v == 1 {
            return -1;
        }

        ret += v / 3;
    }

    ret
}

fn main() {
    let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
    let ret = minimum_rounds(tasks);
    println!("ret={ret}");
}

#[test]
fn test_minimum_rounds() {
    {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        assert_eq!(minimum_rounds(tasks), 4);
    }
    {
        let tasks = vec![2, 3, 3];
        assert_eq!(minimum_rounds(tasks), -1);
    }

    {
        let tasks = vec![
            66, 66, 63, 61, 63, 63, 64, 66, 66, 65, 66, 65, 61, 67, 68, 66, 62, 67, 61, 64, 66, 60,
            69, 66, 65, 68, 63, 60, 67, 62, 68, 60, 66, 64, 60, 60, 60, 62, 66, 64, 63, 65, 60, 69,
            63, 68, 68, 69, 68, 61,
        ];
        assert_eq!(minimum_rounds(tasks), 20);
    }
}
