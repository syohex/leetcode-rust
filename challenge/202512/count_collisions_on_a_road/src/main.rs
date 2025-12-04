fn count_collisions(directions: String) -> i32 {
    let mut ret = 0;
    let mut count = -1;

    for c in directions.chars() {
        match c {
            'L' => {
                if count >= 0 {
                    ret += count + 1;
                    count = 0;
                }
            }
            'R' => {
                if count < 0 {
                    count = 1;
                } else {
                    count += 1;
                }
            }
            _ => {
                if count >= 1 {
                    ret += count;
                }

                count = 0;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_collisions("RLRSLL".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_collisions("RLRSLL".to_string()), 5);
    assert_eq!(count_collisions("LLRR".to_string()), 0);
}
