use util::linked_list::*;

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut next = head;
    let mut sum = 0;
    while let Some(p) = next {
        sum = (sum << 1) + p.val;
        next = p.next;
    }

    sum
}

fn main() {
    println!(
        "get_decimal_value([1, 0, 1])={}",
        get_decimal_value(to_list(vec![1, 0, 1]))
    );
}

#[test]
fn test_get_decimal_value() {
    {
        let r = to_list(vec![1, 0, 1]);
        assert_eq!(get_decimal_value(r), 5);
    }
    {
        let r = to_list(vec![0]);
        assert_eq!(get_decimal_value(r), 0);
    }
    {
        let r = to_list(vec![1]);
        assert_eq!(get_decimal_value(r), 1);
    }
    {
        let r = to_list(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(get_decimal_value(r), 18880);
    }
    {
        let r = to_list(vec![0, 0]);
        assert_eq!(get_decimal_value(r), 0);
    }
}
