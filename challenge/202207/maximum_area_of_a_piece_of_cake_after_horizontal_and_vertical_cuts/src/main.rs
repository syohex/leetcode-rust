fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut horizontal_cuts = horizontal_cuts;
    horizontal_cuts.push(0);
    horizontal_cuts.push(h);
    horizontal_cuts.sort_unstable();

    let mut vertical_cuts = vertical_cuts;
    vertical_cuts.push(0);
    vertical_cuts.push(w);
    vertical_cuts.sort_unstable();

    let mut h_max = 0;
    for i in 0..(horizontal_cuts.len() - 1) {
        h_max = std::cmp::max(h_max, horizontal_cuts[i + 1] - horizontal_cuts[i]);
    }

    let mut w_max = 0;
    for i in 0..(vertical_cuts.len() - 1) {
        w_max = std::cmp::max(w_max, vertical_cuts[i + 1] - vertical_cuts[i]);
    }

    ((h_max as i64) * (w_max as i64) % 1_000_000_007) as i32
}

fn main() {
    let horizontal_cuts = vec![1, 2, 4];
    let vertical_cuts = vec![1, 3];
    let ret = max_area(5, 4, horizontal_cuts, vertical_cuts);
    println!("ret={ret}");
}

#[test]
fn test_max_area() {
    {
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];
        let ret = max_area(5, 4, horizontal_cuts, vertical_cuts);
        assert_eq!(ret, 4);
    }
    {
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];
        let ret = max_area(5, 4, horizontal_cuts, vertical_cuts);
        assert_eq!(ret, 6);
    }
    {
        let horizontal_cuts = vec![3];
        let vertical_cuts = vec![3];
        let ret = max_area(5, 4, horizontal_cuts, vertical_cuts);
        assert_eq!(ret, 9);
    }
}
