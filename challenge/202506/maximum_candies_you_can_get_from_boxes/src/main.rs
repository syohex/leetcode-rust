pub fn max_candies(
    status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>,
) -> i32 {
    use std::collections::VecDeque;

    let n = status.len();
    let mut is_opened = vec![];
    for &s in &status {
        is_opened.push(s == 1);
    }

    let mut is_collected = vec![false; n];
    let mut has_box = vec![false; n];

    let mut ret = 0;
    let mut q = VecDeque::new();
    for &init in &initial_boxes {
        let init = init as usize;
        has_box[init] = true;
        if is_opened[init] {
            q.push_back(init);
            is_collected[init] = true;
            ret += candies[init];
        }
    }

    while !q.is_empty() {
        let index = q.pop_front().unwrap();

        for &key in &keys[index] {
            let key = key as usize;
            is_opened[key] = true;
            if !is_collected[key] && has_box[key] {
                q.push_back(key);
                is_collected[key] = true;
                ret += candies[key];
            }
        }

        for &b in &contained_boxes[index] {
            let b = b as usize;
            has_box[b] = true;
            if !is_collected[b] && is_opened[b] {
                q.push_back(b);
                is_collected[b] = true;
                ret += candies[b];
            }
        }
    }

    ret
}
fn main() {
    let status = vec![1, 0, 1, 0];
    let candies = vec![7, 5, 4, 100];
    let keys = vec![vec![], vec![], vec![1], vec![]];
    let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
    let initial_boxes = vec![0];
    let ret = max_candies(status, candies, keys, contained_boxes, initial_boxes);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let status = vec![1, 0, 1, 0];
        let candies = vec![7, 5, 4, 100];
        let keys = vec![vec![], vec![], vec![1], vec![]];
        let contained_boxes = vec![vec![1, 2], vec![3], vec![], vec![]];
        let initial_boxes = vec![0];
        let ret = max_candies(status, candies, keys, contained_boxes, initial_boxes);
        assert_eq!(ret, 16);
    }
    {
        let status = vec![1, 0, 0, 0, 0, 0];
        let candies = vec![1, 1, 1, 1, 1, 1];
        let keys = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let contained_boxes = vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]];
        let initial_boxes = vec![0];
        let ret = max_candies(status, candies, keys, contained_boxes, initial_boxes);
        assert_eq!(ret, 6);
    }
}
