# 学習用に作成した Rust 製アンケート Web アプリ

Web 上で名前と感想などを入力して提出し、サーバで一覧できる  
リアクションペーパのような用途も考え、学籍番号の入力欄がある

## Features

サーバもフロントも Rust で書いた

## Requirements

[cargo-make](https://github.com/sagiegurari/cargo-make)

`cargo install --force cargo-make`

## Usage

`cargo make start --release`

`http://localhost:8000`にアクセス
