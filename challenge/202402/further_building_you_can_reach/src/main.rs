fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut bh = BinaryHeap::new();
    let mut bricks = bricks;
    for i in 0..heights.len() - 1 {
        if heights[i] >= heights[i + 1] {
            continue;
        }

        bh.push(Reverse(heights[i + 1] - heights[i]));
        if bh.len() <= ladders as usize {
            continue;
        }

        if let Some(Reverse(n)) = bh.pop() {
            bricks -= n;
            if bricks < 0 {
                return i as i32;
            }
        }
    }

    heights.len() as i32 - 1
}

fn main() {
    let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
    let ret = furthest_building(heights, 10, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        let ret = furthest_building(heights, 5, 1);
        assert_eq!(ret, 4)
    }
    {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        let ret = furthest_building(heights, 10, 2);
        assert_eq!(ret, 7)
    }
    {
        let heights = vec![14, 3, 19, 3];
        let ret = furthest_building(heights, 17, 0);
        assert_eq!(ret, 3)
    }
}
