// This is based on https://github.com/aylei/leetcode-rust/blob/master/src/util/linked_list.rs

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for n in v.into_iter().rev() {
        let mut node = ListNode::new(n);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}

#[macro_export]
macro_rules! list {
    ($($e:expr), *) => { to_list(vec![$($e.to_owned()), *]) };
    ($($e:expr,) *) => { to_list(vec![$($e.to_owned()), *]) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_list() {
        let lst = to_list(vec![1, 2, 3]);
        assert!(lst.is_some());

        let e1 = lst.as_ref().unwrap();
        assert_eq!(e1.val, 1);

        let e2 = e1.next.as_ref().unwrap();
        assert_eq!(e2.val, 2);

        let e3 = e2.next.as_ref().unwrap();
        assert_eq!(e3.val, 3);
        assert!(e3.next.is_none());
    }

    #[test]
    fn test_list_macro() {
        let lst = to_list(vec![1, 2, 3]);
        assert_eq!(lst, list!(1, 2, 3));
    }
}
