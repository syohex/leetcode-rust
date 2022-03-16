fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];

    let mut i = 0;
    let mut j = 0;
    loop {
        let mut ok = false;

        if i < pushed.len() {
            stack.push(pushed[i]);
            i += 1;
            ok = true;
        }

        while j < popped.len() {
            if !stack.is_empty() && *stack.last().unwrap() == popped[j] {
                stack.pop();
                j += 1;
                ok = true;
                continue;
            }

            break;
        }

        if !ok {
            break;
        }
    }

    i == pushed.len() && j == popped.len()
}

fn main() {
    let pushed = vec![1, 2, 3, 4, 5];
    let poped = vec![4, 5, 3, 2, 1];
    let ret = validate_stack_sequences(pushed, poped);
    println!("ret={ret}");
}

#[test]
fn test_validate_stack_sequences() {
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let poped = vec![4, 5, 3, 2, 1];
        assert!(validate_stack_sequences(pushed, poped));
    }
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let poped = vec![4, 3, 5, 1, 2];
        assert!(!validate_stack_sequences(pushed, poped));
    }
}
