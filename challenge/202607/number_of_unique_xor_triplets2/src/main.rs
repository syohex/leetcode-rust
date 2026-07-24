fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    let mut table_len = 1;
    while table_len <= max {
        table_len <<= 1;
    }

    let len = nums.len();
    let mut doubles = vec![false; table_len as usize];
    for i in 0..len {
        for j in i..len {
            let idx = nums[i] ^ nums[j];
            doubles[idx as usize] = true;
        }
    }

    let mut triples = vec![false; table_len as usize];
    for i in 0..table_len {
        if doubles[i as usize] {
            for &n in &nums {
                let idx = i ^ n;
                triples[idx as usize] = true;
            }
        }
    }

    triples.into_iter().filter(|n| *n).count() as i32
}

fn main() {
    let ret = unique_xor_triplets(vec![1, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(unique_xor_triplets(vec![1, 3]), 2);
    assert_eq!(unique_xor_triplets(vec![6, 7, 8, 9]), 4);
}
