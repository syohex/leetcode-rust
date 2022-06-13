fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut triangle = triangle;

    for i in 1..triangle.len() {
        let len = triangle[i].len();
        for j in 0..len {
            if j == 0 {
                triangle[i][0] += triangle[i - 1][0];
            } else if j == len - 1 {
                triangle[i][j] += triangle[i - 1][j - 1];
            } else {
                triangle[i][j] += std::cmp::min(triangle[i - 1][j - 1], triangle[i - 1][j]);
            }
        }
    }

    *triangle[triangle.len() - 1].iter().min().unwrap()
}

fn main() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let ret = minimum_total(triangle);
    println!("ret={ret}");
}

#[test]
fn test_minimum_total() {
    {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let ret = minimum_total(triangle);
        assert_eq!(ret, 11);
    }
    {
        let triangle = vec![vec![-10]];
        let ret = minimum_total(triangle);
        assert_eq!(ret, -10);
    }
    {
        let triangle = vec![vec![1], vec![2, 3]];
        let ret = minimum_total(triangle);
        assert_eq!(ret, 3);
    }
}
