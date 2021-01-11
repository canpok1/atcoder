# Rustの文法

## [基本データ型](https://doc.rust-jp.rs/rust-by-example-ja/primitives.html)
```
fn main() {
    assert_eq!(10_i32.abs(), 10);  // 絶対値
    assert_eq!(i0_i32.pow(2), 100); // 累乗
    assert_eq!(4.0_f32.sqrt(), 2.0); // 平方根
    assert_eq!(10_f32.powi(2), 100.0); // 累乗
    assert_eq!(10_f32.powf(2.0), 100.0); // 累乗
    assert_eq!(4.4_f32.round(), 4.0); // 四捨五入
    assert_eq!(4.5_f32.round(), 5.0); // 四捨五入
    assert_eq!(4.5_f32.floor(), 4.0); // 切り捨て
    assert_eq!(4.4_f32.ceil(), 5.0); // 切り上げ

    // すべて同じ値の固定長配列
    let array = [5; 3];
    assert_eq!(array.len(), 3);
    assert_eq!(array[0], 5);
    assert_eq!(array[1], 5);
    assert_eq!(array[2], 5);

    assert_eq!(1_000_000u32, 1000000); // 桁区切り
}

```


## [条件分岐/繰り返し](https://doc.rust-jp.rs/rust-by-example-ja/flow_control.html)
```
use std::collections::HashMap;

fn main() {
    // loopの結果を返す
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // 0〜9
    for n in 0..10 {
        println!("n={}", n);
    }
    // 0〜10
    for n in 0..=10 {
        println!("n={}", n);
    }

    // iteratorの各要素（参照）
    let numbers:Vec<i32> = vec![1, 2, 3];
    for n in numbers.iter() {   // for n in &numbers も同様
        println!("{}", n);   // nの型は&i32
    }
    println!("{:?}", numbers);

    // iteratorの各要素（借用）
    let numbers:Vec<i32> = vec![1, 2, 3];
    for n in numbers.into_iter() {  // for n in numbers も同様
        println!("{}", n);   // nの型はi32
    }
    // into_iterでmoveされるのでエラー
    // println!("{:?}", numbers);

    // iteratorの各要素（書き換え）
    let mut numbers:Vec<i32> = vec![1, 2, 3];
    for n in numbers.iter_mut() {
        *n = *n * 2;  // nの型は&i32
    }
    println!("{:?}", numbers); // [2, 4, 6]

    // iteratorの各要素（インデックス付）
    let numbers = vec![1,2,3];
    for (i, n) in numbers.iter().enumerate() {
        println!("numbers[{}]={}", i, n); // nの型は&i32
    }

    let opt = Some(10);
    // matchで存在チェック
    let number = match opt {
        Some(n) => n,
        None => 0
    };
    assert_eq!(number, 10);
    // if letでも書ける
    let number = if let Some(n) = opt {n} else {0};
    assert_eq!(number, 10);
}
```

## [関数](https://doc.rust-jp.rs/rust-by-example-ja/fn.html)
```
fn main() {
    assert_eq!(sum(2,3), 5);
    
    let point = Point::new(3.0, 2.0);
    assert_eq!(point.to_tuple(), (3.0, 2.0));

    let mut point = Point::new(1.0, 1.0);
    point.set(10.0, 100.0);
    assert_eq!(point.to_tuple(), (10.0, 100.0));
}

fn sum(v1:i32, v2:i32) -> i32 {
    v1 + v2
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn to_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
```

## [標準ライブラリの型](https://doc.rust-jp.rs/rust-by-example-ja/std.html)
```
use std::collections::HashMap;

fn main() {
    // ベクタ型
    let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![0; 5];   // vec![0,0,0,0,0] と同じ
    let v: Vec<i32> = (0..3).collect(); // vec![0,1,2] と同じ
    assert_eq!(v[2], 2);

    // String型
    let s: String = String::from("abc");
    for c in s.chars() {
        println!("{}", c);  // 1文字ずつ表示される
    }

    // HashMap型
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    for (k, v) in map.iter() {
        println!("map[{}]={}", k, v);   // 順不同でkey/valueが表示される
    }
}
```

## 関連リンク

* [stdクレートのリファレンス](https://doc.rust-lang.org/std/)
* [Rust by Example日本語版](https://doc.rust-jp.rs/rust-by-example-ja/index.html)
