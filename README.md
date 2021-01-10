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

### テスト

[cargo-atcoder](https://github.com/tanakh/cargo-atcoder)で各種操作が可能
```
# テスト
cargo atcoder test a
```

