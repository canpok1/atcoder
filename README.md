# atcoder-rust

AtCoder挑戦用のリポジトリ


## 環境構築

1. リポジトリをclone
2. cloneしたディレクトリをVSCodeのRemote Containerで開く
3. AtCoderにログイン(VSCodeの統合ターミナルで `cargo atcoder login`)


## 各種操作

### コンテスト開始

1. セットアップ用スクリプトを実行 (VSCode統合ターミナルで`./settup-contest.sh <<コンテストのトップのURL>>`)
2. コンテスト用ディレクトリをVSCodeのワークスペースに追加
3. 実装開始

### よく使うコマンド

```
# テスト実行（問題ページの入力例）
cargo atcoder test a

# テスト実行（自作のテストコード）
cargo test --bin a

# テスト実行（手動入力）
cargo atcoder test a --custom

# 提出
cargo atcoder submit a
```

### その他
[cargo-atcoder](https://github.com/tanakh/cargo-atcoder)を参照


## 自作メモ
* [Rustチートシート](./docs/cheatsheet.md)
* [アルゴリズム](./docs/algorithm.md)


## リンク
* [AtCoder](https://atcoder.jp/?lang=ja)
