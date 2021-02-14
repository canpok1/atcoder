use cargo_snippet::snippet;

#[snippet(name = "myfunc binary_search")]
#[allow(dead_code)]
fn binary_search<F>(mut ng_idx: i64, mut ok_idx: i64, is_ok: F) -> (i64, i64)
where
    F: Fn(i64) -> bool,
{
    while (ok_idx - ng_idx).abs() > 1 {
        let mid_idx = (ok_idx + ng_idx) / 2;
        if is_ok(mid_idx) {
            ok_idx = mid_idx;
        } else {
            ng_idx = mid_idx;
        }
    }
    (ng_idx, ok_idx)
}

#[test]
fn test_binary_search_1() {
    let array = vec![0, 0, 1, 1, 2, 2, 3, 3];
    let (ng_idx, ok_idx) = binary_search(0, array.len() as i64, |v| {
        let idx = v as usize;
        array[idx] > 1
    });
    assert_eq!(ng_idx, 3);
    assert_eq!(ok_idx, 4);
}

#[test]
fn test_binary_search_2() {
    let (ng_idx, ok_idx) = binary_search(0, 10, |v| v > 1);
    assert_eq!(ng_idx, 1);
    assert_eq!(ok_idx, 2);
}

#[test]
fn test_binary_search_3() {
    let (ng_idx, ok_idx) = binary_search(10, 0, |v| v < 5);
    assert_eq!(ng_idx, 5);
    assert_eq!(ok_idx, 4);
}
