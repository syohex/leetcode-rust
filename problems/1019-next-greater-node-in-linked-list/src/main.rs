use util::linked_list::*;
use util::*;

fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut p = head;
    while let Some(node) = p {
        v.push(node.val);
        p = node.next;
    }

    let mut ret: Vec<i32> = vec![];
    let mut stack: Vec<i32> = vec![];
    for &n in v.iter().rev() {
        while !stack.is_empty() && stack[stack.len() - 1] <= n {
            stack.pop();
        }

        if stack.is_empty() {
            ret.push(0);
        } else {
            ret.push(stack[stack.len() - 1]);
        }

        stack.push(n.to_owned());
    }

    ret.reverse();
    ret
}

fn main() {
    println!(
        "next_larger_nodes([2, 1, 5])={:?}",
        next_larger_nodes(list!(2, 1, 5))
    );
}

#[test]
fn test_next_larger_nodes() {
    assert_eq!(next_larger_nodes(list!(2, 1, 5)), vec![5, 5, 0]);
    assert_eq!(next_larger_nodes(list!(3, 3)), vec![0, 0]);
    assert_eq!(next_larger_nodes(list!(2, 7, 4, 3, 5)), vec![7, 0, 5, 5, 0]);
    assert_eq!(
        next_larger_nodes(list!(1, 7, 5, 1, 9, 2, 5, 1)),
        vec![7, 9, 9, 9, 0, 5, 0, 0]
    );
}
