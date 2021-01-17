fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mins: Vec<&i32> = rectangles.iter().map(|v| v.iter().min().unwrap()).collect();
    let max = mins.iter().max().unwrap();

    mins.iter().filter(|n| ***n == **max).count() as i32
}

fn main() {
    let input = vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]];
    println!(
        "count_good_rectangles({:?}) = {}",
        input,
        count_good_rectangles(input.clone())
    );
}

#[test]
fn test_count_good_rectangles() {
    {
        let input = vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]];
        assert_eq!(count_good_rectangles(input), 3);
    }
    {
        let input = vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]];
        assert_eq!(count_good_rectangles(input), 3);
    }
}
