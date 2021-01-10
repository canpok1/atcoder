use std::collections::HashMap;

fn main() {
    let mut stdin: Vec<String> = Vec::new();
    loop {
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).unwrap();
        if size == 0 {
            break;
        }
        stdin.push(s.trim_end().to_string());
    }

    for o in run(&stdin) {
        println!("{}", o);
    }
}

fn run(stdin: &Vec<String>) -> Vec<String> {
    let mut buf = Vec::new();

    let n: usize = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        ws.next().unwrap().parse().unwrap()
    };

    let mut cities: Vec<(i32, i32, i32)> = Vec::new();
    for (i, v) in stdin.iter().enumerate() {
        if i == 0 {
            continue;
        }
        buf.push(format!("{}", "hello world!"));
        cities.push({
            let mut ws = v.trim_end().split_whitespace();
            let x: i32 = ws.next().unwrap().parse().unwrap();
            let y: i32 = ws.next().unwrap().parse().unwrap();
            let z: i32 = ws.next().unwrap().parse().unwrap();
            (x, y, z)
        });
    }

    // dp[i][S] = 現在地が都市 i で、訪問済みの街の集合が S であるときのコストの最小値
    let mut dp: Vec<Vec<i32>> = Vec::new();
    (0..n).for_each(|_i| {
        dp.push(Vec::new());
    });

    let s_max: usize = (1 << n) - 1;
    (0..=s_max).for_each(|s| {
        // j -> i のルートの最小コストを算出
        (0..n).for_each(|i| {
            let mut min = i32::MAX;
            // 到達不可？
            if ((s >> i) & 1) != 1 {
                dp[i as usize].push(min);
                return;
            }
            // スタート地点？
            if i == 0 {
                dp[i as usize].push(0);
                return;
            }

            (0..n).for_each(|j| {
                // 未訪問？
                if ((s >> j) & 1) != 1 {
                    return;
                }

                let (x1, y1, z1) = cities[i as usize];
                let (x2, y2, z2) = cities[j as usize];
                let dist_cost = (x1 - x2).abs() + (y1 - y2).abs() + std::cmp::max(0, z1 - z2);

                let before_s = s & !(1 << j);
                let before_cost = dp[i as usize][before_s as usize];
                if min > (dist_cost + before_cost) {
                    min = dist_cost + before_cost;
                }
            });

            dp[i as usize][s as usize] = min;
        });
    });

    let mut ans = std::i32::MAX;
    (0..n).for_each(|i| {
        let (x1, y1, z1) = cities[0];
        let (x2, y2, z2) = cities[i as usize];
        let dist_cost = (x1 - x2).abs() + (y1 - y2).abs() + std::cmp::max(0, z1 - z2);
        let cost = dist_cost + dp[i as usize][s_max];
        if ans > cost {
            ans = cost;
        }
    });
    buf.push(format!("{}", ans));

    buf
}

#[test]
fn test_sample_1() {
    let stdin = ["2", "0 0 0", "1 2 3"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(&stdin), vec!("9"));
}

#[test]
fn test_sample_2() {
    let stdin = ["3", "0 0 0", "1 1 1", "-1 -1 -1"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(&stdin), vec!("10"));
}

#[test]
fn test_sample_3() {
    let stdin = [
        "17",
        "14142 13562 373095",
        "-17320 508075 68877",
        "223606 -79774 9979",
        "-24494 -89742 783178",
        "26457 513110 -64591",
        "-282842 7124 -74619",
        "31622 -77660 -168379",
        "-33166 -24790 -3554",
        "346410 16151 37755",
        "-36055 51275 463989",
        "37416 -573867 73941",
        "-3872 -983346 207417",
        "412310 56256 -17661",
        "-42426 40687 -119285",
        "43588 -989435 -40674",
        "-447213 -59549 -99579",
        "45825 7569 45584",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect();

    assert_eq!(run(&stdin), vec!("6519344"));
}
