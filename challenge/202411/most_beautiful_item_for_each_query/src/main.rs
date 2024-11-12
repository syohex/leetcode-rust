fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut items = items;
    items.sort_unstable_by(|a, b| {
        if a[0] == b[0] {
            b[1].cmp(&a[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut max_val = 0;
    for i in 0..items.len() {
        max_val = std::cmp::max(max_val, items[i][1]);
        items[i][1] = max_val;
    }

    let mut ret = vec![];
    for q in queries {
        let mut left = 0i32;
        let mut right = items.len() as i32 - 1;
        let mut v = 0;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            if items[mid][0] <= q {
                v = std::cmp::max(v, items[mid][1]);
                left = mid as i32 + 1;
            } else {
                right = mid as i32 - 1;
            }
        }

        ret.push(v);
    }

    ret
}

fn main() {
    let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
    let queries = vec![1, 2, 3, 4, 5, 6];
    let ret = maximum_beauty(items, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
        let queries = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 4, 5, 5, 6, 6];
        let ret = maximum_beauty(items, queries);
        assert_eq!(ret, expected);
    }
    {
        let items = vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]];
        let queries = vec![1];
        let expected = vec![4];
        let ret = maximum_beauty(items, queries);
        assert_eq!(ret, expected);
    }
    {
        let items = vec![vec![10, 1000]];
        let queries = vec![5];
        let expected = vec![0];
        let ret = maximum_beauty(items, queries);
        assert_eq!(ret, expected);
    }
    {
        let items = vec![
            vec![193, 732],
            vec![781, 962],
            vec![864, 954],
            vec![749, 627],
            vec![136, 746],
            vec![478, 548],
            vec![640, 908],
            vec![210, 799],
            vec![567, 715],
            vec![914, 388],
            vec![487, 853],
            vec![533, 554],
            vec![247, 919],
            vec![958, 150],
            vec![193, 523],
            vec![176, 656],
            vec![395, 469],
            vec![763, 821],
            vec![542, 946],
            vec![701, 676],
        ];
        let queries = vec![
            885, 1445, 1580, 1309, 205, 1788, 1214, 1404, 572, 1170, 989, 265, 153, 151, 1479,
            1180, 875, 276, 1584,
        ];
        let expected = vec![
            962, 962, 962, 962, 746, 962, 962, 962, 946, 962, 962, 919, 746, 746, 962, 962, 962,
            919, 962,
        ];
        let ret = maximum_beauty(items, queries);
        assert_eq!(ret, expected);
    }
}
