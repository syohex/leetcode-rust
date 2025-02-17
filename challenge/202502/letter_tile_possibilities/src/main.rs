fn num_tile_possibilities(tiles: String) -> i32 {
    fn f(table: &mut Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..26 {
            if table[i] == 0 {
                continue;
            }

            ret += 1;

            table[i] -= 1;
            ret += f(table);
            table[i] += 1;
        }

        ret
    }

    let mut table = vec![0; 26];
    for b in tiles.bytes() {
        let index = (b - b'A') as usize;
        table[index] += 1;
    }

    f(&mut table)
}

fn main() {
    let tiles = "AAABBC".to_string();
    let ret = num_tile_possibilities(tiles);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let tiles = "AAB".to_string();
        let ret = num_tile_possibilities(tiles);
        assert_eq!(ret, 8);
    }
    {
        let tiles = "AAABBC".to_string();
        let ret = num_tile_possibilities(tiles);
        assert_eq!(ret, 188);
    }
    {
        let tiles = "V".to_string();
        let ret = num_tile_possibilities(tiles);
        assert_eq!(ret, 1);
    }
}
