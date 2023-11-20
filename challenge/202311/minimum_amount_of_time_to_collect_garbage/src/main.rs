fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    fn to_vec(s: &str) -> Vec<i32> {
        s.chars().fold(vec![0; 3], |mut acc, c| {
            match c {
                'M' => acc[0] += 1,
                'P' => acc[1] += 1,
                'G' => acc[2] += 1,
                _ => panic!("never reach here"),
            }

            acc
        })
    }

    let gs: Vec<Vec<i32>> = garbage.iter().map(|s| to_vec(s)).collect();
    let mut lasts = vec![0; 3];
    for (i, g) in gs.iter().enumerate() {
        if g[0] != 0 {
            lasts[0] = i;
        }
        if g[1] != 0 {
            lasts[1] = i;
        }
        if g[2] != 0 {
            lasts[2] = i;
        }
    }

    let mut ret = 0;
    for i in 0..3 {
        for (j, g) in gs.iter().enumerate() {
            ret += g[i];

            if lasts[i] == j {
                break;
            }

            ret += travel[j];
        }
    }

    ret
}

fn main() {
    let garbage = vec![
        "G".to_string(),
        "P".to_string(),
        "GP".to_string(),
        "GG".to_string(),
    ];
    let travel = vec![2, 4, 3];
    let ret = garbage_collection(garbage, travel);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let garbage = vec![
            "G".to_string(),
            "P".to_string(),
            "GP".to_string(),
            "GG".to_string(),
        ];
        let travel = vec![2, 4, 3];
        let ret = garbage_collection(garbage, travel);
        assert_eq!(ret, 21);
    }
    {
        let garbage = vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()];
        let travel = vec![3, 10];
        let ret = garbage_collection(garbage, travel);
        assert_eq!(ret, 37);
    }
}
