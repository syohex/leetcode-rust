fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut glasses = vec![vec![0.0; 101]; 101];

    let query_row = query_row as usize;
    let query_glass = query_glass as usize;

    glasses[0][0] = poured as f64;
    for i in 0..=query_row {
        for j in 0..=query_glass {
            let v = glasses[i][j] - 1.0;
            if v > 0.0 {
                let half = v / 2.0;
                glasses[i + 1][j] += half;
                glasses[i + 1][j + 1] += half;
            }
        }
    }

    if glasses[query_row][query_glass] >= 1.0 {
        1.0
    } else {
        glasses[query_row][query_glass]
    }
}

fn main() {
    let ret = champagne_tower(100000009, 33, 17);
    println!("ret={ret}");
}

#[test]
fn test_champagne_tower() {
    assert_eq!(champagne_tower(1, 1, 1), 0.0);
    assert_eq!(champagne_tower(2, 1, 1), 0.5);
    assert_eq!(champagne_tower(100000009, 33, 17), 1.0);
}
