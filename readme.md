# Neknaj Circuit Game Tutorial

NCG (Neknaj Circuit Game) のチュートリアルサイトを生成するためのツールです。  

## サイト

https://neknaj.github.io/circuitgame_tutorial/0.はじめに.html で読むことが出来ます  

## 必要要件

- Rust 1.70以上
- Cargo

## プロジェクト構造

```
circuitgame_tutorial/
├── builder/     # Static site generator
├── src/         # チュートリアルのMarkdownファイル
├── templates/   # HTMLテンプレート
└── dist/        # 生成されたサイト
```

## ローカルでの開発

1. リポジトリのクローン:  
   ```bash
   git clone https://github.com/neknaj/circuitgame_tutorial.git
   cd circuitgame_tutorial
   ```

2. チュートリアルのビルド:  
   ```bash
   cargo run --manifest-path ./builder/Cargo.toml
   ```

## チュートリアルの編集

- `src/` ディレクトリ内のMarkdownファイルを編集  
- フォルダ構造に応じて自動的にナビゲーションが生成されます  
- ビルド後、`dist/` ディレクトリに生成されたHTMLファイルが配置されます  

## ライセンス

MIT License