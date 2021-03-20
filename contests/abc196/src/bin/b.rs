use proconio::input;
fn main() {
    input! {
        x: String,
    };

    let splited: Vec<&str> = x.split(".").collect();
    println!("{}", splited[0])
}
