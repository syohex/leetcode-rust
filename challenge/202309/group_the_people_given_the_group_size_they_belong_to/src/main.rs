fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;

    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, group) in group_sizes.into_iter().enumerate() {
        h.entry(group).or_insert(vec![]).push(i as i32);
    }

    let mut ret = vec![];
    for (k, v) in h.into_iter() {
        let chunks: Vec<Vec<i32>> = v.chunks(k as usize).map(|cs| cs.to_vec()).collect();
        for cs in chunks {
            ret.push(cs);
        }
    }

    ret
}

fn main() {
    let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
    let ret = group_the_people(group_sizes);
    println!("ret={ret:?}");
}

#[test]
fn test_group_the_people() {
    {
        let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
        let ret = group_the_people(group_sizes);
        assert_eq!(ret.len(), 3);
        assert!(ret.contains(&vec![5]));
        assert!(ret.contains(&vec![0, 1, 2]));
        assert!(ret.contains(&vec![3, 4, 6]));
    }
    {
        let group_sizes = vec![2, 1, 3, 3, 3, 2];
        let ret = group_the_people(group_sizes);
        assert_eq!(ret.len(), 3);
        assert!(ret.contains(&vec![1]));
        assert!(ret.contains(&vec![0, 5]));
        assert!(ret.contains(&vec![2, 3, 4]));
    }
}
