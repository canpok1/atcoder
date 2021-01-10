#!/bin/bash
# コンテスト毎のプロジェクトを生成する
# $1: コンテストページのURL(例: https://atcoder.jp/contests/abc152)
set -eu
cd $(dirname ${0})

CONTESTS_DIR="./contests/"
cd ${CONTESTS_DIR}

CONTEST_NAME=${1#https://atcoder.jp/contests/} 
echo "CONTEST NAME: ${CONTEST_NAME}"

cargo atcoder new ${CONTEST_NAME}
echo "open contest directory: $(pwd)/${CONTEST_NAME}"
