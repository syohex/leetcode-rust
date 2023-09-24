fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let query_row = query_row as usize;
    let query_glass = query_glass as usize;

    let mut glasses = vec![vec![0.0; query_glass + 2]; 102];
    glasses[0][0] = poured as f64;

    for i in 0..=query_row {
        for j in 0..=query_glass {
            let overflowed = glasses[i][j] - 1.0;
            if overflowed > 0.0 {
                let half = overflowed / 2.0;
                glasses[i + 1][j] += half;
                glasses[i + 1][j + 1] += half;
            }
        }
    }

    if glasses[query_row][query_glass] > 1.0 {
        1.0
    } else {
        glasses[query_row][query_glass]
    }
}

fn main() {
    let ret = champagne_tower(2, 1, 1);
    println!("ret={ret}");
}

#[test]
fn test_champagne_tower() {
    {
        let ret = champagne_tower(1, 1, 1);
        assert_eq!(ret, 0.0);
    }
    {
        let ret = champagne_tower(2, 1, 1);
        assert_eq!(ret, 0.5);
    }
    {
        let ret = champagne_tower(100000009, 33, 17);
        assert_eq!(ret, 1.0);
    }
}
