fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut v: Vec<(String, i32)> = names.into_iter().zip(heights.into_iter()).collect();
    v.sort_by(|(_, height1), (_, height2)| height2.cmp(height1));

    v.into_iter().map(|(name, _)| name).collect()
}

fn main() {
    let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights = vec![180, 165, 170];
    let ret = sort_people(names, heights);
    println!("ret={:?}", ret);
}

#[test]
fn test_sort_people() {
    {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        let expected = vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()];
        let ret = sort_people(names, heights);
        assert_eq!(ret, expected);
    }
    {
        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155, 185, 150];
        let expected = vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()];
        let ret = sort_people(names, heights);
        assert_eq!(ret, expected);
    }
}
