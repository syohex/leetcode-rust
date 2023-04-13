fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack: Vec<i32> = vec![];
    let mut i = 0usize;
    let mut j = 0usize;

    while i < pushed.len() || j < pushed.len() {
        if let Some(v) = stack.last() {
            if *v == popped[j] {
                stack.pop();
                j += 1;
                continue;
            }
        }

        if i >= pushed.len() {
            return false;
        }

        stack.push(pushed[i]);
        i += 1;
    }

    i == pushed.len() && j == popped.len()
}

fn main() {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    let ret = validate_stack_sequences(pushed, popped);
    println!("ret={ret}");
}

#[test]
fn test_validate_stack_sequences() {
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        let ret = validate_stack_sequences(pushed, popped);
        assert!(ret);
    }
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        let ret = validate_stack_sequences(pushed, popped);
        assert!(!ret);
    }
}
