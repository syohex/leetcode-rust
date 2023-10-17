fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    use std::collections::HashSet;

    fn find_root(n: i32, children: &HashSet<i32>) -> i32 {
        for i in 0..n {
            if !children.contains(&i) {
                return i;
            }
        }

        -1
    }

    let mut children = HashSet::new();
    for &child in &left_child {
        children.insert(child);
    }
    for &child in &right_child {
        children.insert(child);
    }

    let root = find_root(n, &children);
    if root == -1 {
        return false;
    }
    dbg!(&children, &root);

    let mut stack = vec![root];
    let mut checked = HashSet::new();

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        checked.insert(node);

        let cs = [left_child[node as usize], right_child[node as usize]];
        for next in cs {
            if next != -1 {
                if checked.contains(&next) {
                    return false;
                }

                stack.push(next);
            }
        }
    }

    checked.len() == n as usize
}

fn main() {
    let left_child = vec![1, -1, 3, -1];
    let right_child = vec![2, -1, -1, -1];
    let ret = validate_binary_tree_nodes(4, left_child, right_child);
    println!("ret={ret}");
}

#[test]
fn test_validate_binary_tree_nodes() {
    {
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, -1, -1, -1];
        let ret = validate_binary_tree_nodes(4, left_child, right_child);
        assert!(ret);
    }
    {
        let left_child = vec![1, -1, 3, -1];
        let right_child = vec![2, 3, -1, -1];
        let ret = validate_binary_tree_nodes(4, left_child, right_child);
        assert!(!ret);
    }
    {
        let left_child = vec![1, 0];
        let right_child = vec![-1, -1];
        let ret = validate_binary_tree_nodes(2, left_child, right_child);
        assert!(!ret);
    }
    {
        let left_child = vec![3, -1, 1, -1];
        let right_child = vec![-1, -1, 0, -1];
        let ret = validate_binary_tree_nodes(4, left_child, right_child);
        assert!(ret);
    }
}
