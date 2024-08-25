## [The Rust Programming Language](https://doc.rust-jp.rs/book-ja/) を読む

### 1章: 事始め
#### 1-1. インストール
- `rustup`: Rustのバージョンと関連ツールの管理
#### 1-2. Hello, World!
- ファイルのコンパイルは`rustc main.rs`で行える
- !で終わるもの(`function_name!()`)ではマクロが呼び出されている
#### 1-3. Hello, Cargo!
- Cargo: Rustのビルドシステム、パッケージマネージャー
- `cargo new project_name`を実行すると、`Cargo.toml`および`src`ディレクトリが作成される
- `Cargo.toml`はTOML形式で書かれており、コンパイルのための設定情報や、プロジェクトの依存を列挙する
- Rustのコードのパッケージを、**クレート**(**crate**)という
- プロジェクトのビルドは`cargo build`、実行ファイルの実行までをまとめて実行するには`cargo run`とする
- `cargo check`とすると、高速にコンパイルできるかどうかのチェックを行う