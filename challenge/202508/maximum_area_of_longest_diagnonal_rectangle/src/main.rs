fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let mut max_diagonal = 0f64;

    for d in dimensions {
        let diagonal = (d[0].pow(2) as f64 + d[1].pow(2) as f64).sqrt();
        if max_diagonal < diagonal {
            max_diagonal = diagonal;
            ret = d[0] * d[1];
        } else if max_diagonal == diagonal {
            ret = std::cmp::max(ret, d[0] * d[1]);
        }
    }
    ret
}

fn main() {
    let dimensions = vec![vec![9, 3], vec![8, 6]];
    let ret = area_of_max_diagonal(dimensions);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let dimensions = vec![vec![9, 3], vec![8, 6]];
        let ret = area_of_max_diagonal(dimensions);
        assert_eq!(ret, 48);
    }
    {
        let dimensions = vec![vec![3, 4], vec![3, 4]];
        let ret = area_of_max_diagonal(dimensions);
        assert_eq!(ret, 12);
    }
}
