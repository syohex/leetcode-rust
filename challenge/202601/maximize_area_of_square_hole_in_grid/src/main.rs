fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
    let mut h_bars = h_bars;
    let mut v_bars = v_bars;

    h_bars.sort_unstable();
    v_bars.sort_unstable();

    let mut max_h_len = 1;
    let mut cur_h_len = 1;
    for i in 1..h_bars.len() {
        if h_bars[i] == h_bars[i - 1] + 1 {
            cur_h_len += 1;
        } else {
            cur_h_len = 1;
        }

        max_h_len = std::cmp::max(max_h_len, cur_h_len);
    }

    let mut max_v_len = 1;
    let mut cur_v_len = 1;
    for i in 1..v_bars.len() {
        if v_bars[i] == v_bars[i - 1] + 1 {
            cur_v_len += 1;
        } else {
            cur_v_len = 1;
        }

        max_v_len = std::cmp::max(max_v_len, cur_v_len);
    }

    let min_len = std::cmp::min(max_h_len, max_v_len) + 1;
    min_len * min_len
}

fn main() {
    let h_bars = vec![2, 3];
    let v_bars = vec![2, 4];
    let ret = maximize_square_hole_area(2, 3, h_bars, v_bars);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let h_bars = vec![3, 2, 4];
        let v_bars = vec![3, 2];
        let ret = maximize_square_hole_area(3, 2, h_bars, v_bars);
        assert_eq!(ret, 9);
    }
    {
        let h_bars = vec![2, 3];
        let v_bars = vec![2];
        let ret = maximize_square_hole_area(2, 1, h_bars, v_bars);
        assert_eq!(ret, 4);
    }
    {
        let h_bars = vec![2];
        let v_bars = vec![2];
        let ret = maximize_square_hole_area(1, 1, h_bars, v_bars);
        assert_eq!(ret, 4);
    }
    {
        let h_bars = vec![2, 3];
        let v_bars = vec![2, 4];
        let ret = maximize_square_hole_area(2, 3, h_bars, v_bars);
        assert_eq!(ret, 4);
    }
}
