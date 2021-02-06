#!/bin/bash
set -eu
cd $(dirname $0)
cargo snippet -t vscode > ../.vscode/myfunc.code-snippets
