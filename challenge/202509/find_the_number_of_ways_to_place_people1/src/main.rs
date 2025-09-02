fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();

    let mut ret = 0;
    for i in 0..len {
        let p1 = &points[i];
        for j in 0..len {
            if i == j {
                continue;
            }

            let p2 = &points[j];
            if !(p1[0] <= p2[0] && p1[1] >= p2[1]) {
                continue;
            }

            if len == 2 {
                ret += 1;
                continue;
            }

            let mut invalid = false;
            for k in 0..len {
                if i == k || j == k {
                    continue;
                }

                let p3 = &points[k];
                if p1[0] <= p3[0] && p3[0] <= p2[0] && p2[1] <= p3[1] && p3[1] <= p1[1] {
                    invalid = true;
                    break;
                }
            }
            if !invalid {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
    let ret = number_of_pairs(points);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let ret = number_of_pairs(points);
        assert_eq!(ret, 0);
    }
    {
        let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
        let ret = number_of_pairs(points);
        assert_eq!(ret, 2);
    }
    {
        let points = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
        let ret = number_of_pairs(points);
        assert_eq!(ret, 2);
    }
}
