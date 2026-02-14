fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let (rows, cols) = (query_row as usize, query_glass as usize);
    let mut glasses = vec![vec![0.0; cols + 2]; rows + 2];
    glasses[0][0] = poured as f64;

    for r in 0..=rows {
        for c in 0..=cols {
            if glasses[r][c] > 1.0 {
                let half = (glasses[r][c] - 1.0) / 2.0;
                glasses[r + 1][c] += half;
                glasses[r + 1][c + 1] += half;
            }
        }
    }

    if glasses[rows][cols] > 1.0 {
        1.0
    } else {
        glasses[rows][cols]
    }
}

fn main() {
    let ret = champagne_tower(100000009, 33, 17);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(champagne_tower(2, 1, 1), 0.5);
    assert_eq!(champagne_tower(1, 1, 1), 0.0);
    assert_eq!(champagne_tower(100000009, 33, 17), 1.0);
}
