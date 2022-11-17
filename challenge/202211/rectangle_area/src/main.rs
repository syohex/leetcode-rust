pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    let area1 = (ax2 - ax1) * (ay2 - ay1);
    let area2 = (bx2 - bx1) * (by2 - by1);

    let overwrap_width = std::cmp::min(ax2, bx2) - std::cmp::max(ax1, bx1);
    let overwrap_height = std::cmp::min(ay2, by2) - std::cmp::max(ay1, by1);

    let overwrap_area = if overwrap_width > 0 && overwrap_height > 0 {
        overwrap_width * overwrap_height
    } else {
        0
    };

    area1 + area2 - overwrap_area
}

fn main() {
    let ax1 = -3;
    let ay1 = 0;
    let ax2 = 3;
    let ay2 = 4;
    let bx1 = 0;
    let by1 = -1;
    let bx2 = 9;
    let by2 = 2;

    let ret = compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
    println!("ret={ret}");
}

#[test]
fn test_compute_area() {
    {
        let ax1 = -3;
        let ay1 = 0;
        let ax2 = 3;
        let ay2 = 4;
        let bx1 = 0;
        let by1 = -1;
        let bx2 = 9;
        let by2 = 2;

        let ret = compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        assert_eq!(ret, 45);
    }
    {
        let ax1 = -2;
        let ay1 = -2;
        let ax2 = 2;
        let ay2 = 2;
        let bx1 = -2;
        let by1 = -2;
        let bx2 = 2;
        let by2 = 2;

        let ret = compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        assert_eq!(ret, 16);
    }
}
