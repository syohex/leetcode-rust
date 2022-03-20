fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    fn f(val: i32, tops: &[i32], bottoms: &[i32]) -> i32 {
        let mut top_rotations = 0;
        let mut bottom_rotations = 0;

        for (&t, &b) in tops.iter().zip(bottoms.iter()) {
            if t != val && b != val {
                return -1;
            }

            if t != val {
                top_rotations += 1;
            } else if b != val {
                bottom_rotations += 1;
            }
        }

        std::cmp::min(top_rotations, bottom_rotations)
    }

    let ret = f(tops[0], &tops, &bottoms);
    if ret != -1 || tops[0] == bottoms[0] {
        return ret;
    }

    f(bottoms[0], &bottoms, &tops)
}

fn main() {
    let tops = vec![2, 1, 2, 4, 2, 2];
    let bottoms = vec![5, 2, 6, 2, 3, 2];
    let ret = min_domino_rotations(tops, bottoms);
    println!("ret={ret}");
}

#[test]
fn test_min_domino_rotations() {
    {
        let tops = vec![2, 1, 2, 4, 2, 2];
        let bottoms = vec![5, 2, 6, 2, 3, 2];
        assert_eq!(min_domino_rotations(tops, bottoms), 2);
    }
    {
        let tops = vec![3, 5, 1, 2, 3];
        let bottoms = vec![3, 6, 3, 3, 4];
        assert_eq!(min_domino_rotations(tops, bottoms), -1);
    }
}
