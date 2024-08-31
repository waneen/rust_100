# 環境構築

## Vscodeのインストール
 - https://www602.math.ryukoku.ac.jp/Prog1/vscode-win.html

## Dockerのインストール
 - https://qiita.com/hoshimado/items/51c99ccaee3d4222d99d

## コンテナのビルドと起動
 全部WSL上で
 - clone
    `git clone https://github.com/waneen/rust_100.git`
 - 移動
    `cd rust_100`
 - コンテナのビルドと起動
    `docker-compose up -d --build`
 - プロジェクトを開く
    `code .`
 - コンテナに入って作業
 　右下の青枠で`WSL: Ubuntu`等となっているところをクリック、`Reopen in Container`で完了