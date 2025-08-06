fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
    fn build_segment_tree(
        pos: usize,
        left: usize,
        right: usize,
        baskets: &[i32],
        tree: &mut Vec<i32>,
    ) {
        if left >= right {
            tree[pos] = baskets[left];
            return;
        }

        let mid = (left + right) / 2;
        let next_left = mid * 2 + 1;
        let next_right = next_left + 1;

        build_segment_tree(next_left, left, mid, baskets, tree);
        build_segment_tree(next_right, mid + 1, right, baskets, tree);

        tree[pos] = std::cmp::max(tree[next_left], tree[next_right]);
    }

    fn has_place_in_segment_tree(
        tree: &mut [i32],
        val: i32,
        pos: usize,
        left: usize,
        right: usize,
    ) -> bool {
        if tree[pos] < val {
            return false;
        }

        if left == right {
            tree[pos] = -1;
            return true;
        }

        let mid = (left + right) / 2;
        let next_left = mid * 2 + 1;
        let next_right = next_left + 1;

        let ok = if tree[next_left] >= val {
            has_place_in_segment_tree(tree, val, next_left, left, mid)
        } else {
            has_place_in_segment_tree(tree, val, next_right, mid + 1, right)
        };

        tree[pos] = std::cmp::max(tree[next_left], tree[next_right]);
        ok
    }

    let len = fruits.len();
    let mut tree = vec![0; len * 2];
    build_segment_tree(0, 0, len - 1, &baskets, &mut tree);

    let mut ret = 0;
    for fruit in fruits {
        if !has_place_in_segment_tree(&mut tree, fruit, 0, 0, len - 1) {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let fruits = vec![4, 2, 5];
    let baskets = vec![3, 5, 4];
    let ret = num_of_unplaced_fruits(fruits, baskets);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        let ret = num_of_unplaced_fruits(fruits, baskets);
        assert_eq!(ret, 1);
    }
    {
        let fruits = vec![3, 6, 1];
        let baskets = vec![6, 4, 7];
        let ret = num_of_unplaced_fruits(fruits, baskets);
        assert_eq!(ret, 0);
    }
}
