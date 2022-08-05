fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0u128; (target + 1) as usize];
    dp[0] = 1;

    for i in 1..=target {
        for num in &nums {
            if i - num >= 0 {
                dp[i as usize] += dp[(i - num) as usize];
            }
        }
    }

    dp[target as usize] as i32
}

fn main() {
    let nums = vec![
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];
    let ret = combination_sum4(nums, 10);
    println!("ret={ret}");
}

#[test]
fn test_combination_sum4() {
    {
        let nums = vec![1, 2, 3];
        let ret = combination_sum4(nums, 4);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![9];
        let ret = combination_sum4(nums, 3);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![
            3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        ];
        let ret = combination_sum4(nums, 10);
        assert_eq!(ret, 9);
    }
    {
        let nums = vec![
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190,
            200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 330, 340, 350, 360,
            370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500, 510, 520, 530,
            540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 660, 670, 680, 690, 700,
            710, 720, 730, 740, 750, 760, 770, 780, 790, 800, 810, 820, 830, 840, 850, 860, 870,
            880, 890, 900, 910, 920, 930, 940, 950, 960, 970, 980, 990, 111,
        ];
        let ret = combination_sum4(nums, 999);
        assert_eq!(ret, 1);
    }
}
