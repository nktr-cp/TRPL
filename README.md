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
- `cargo check`とすると、コンパイルできるかどうかのチェックを高速に行う

### 2章: 数当てゲームのプログラミング
- Rustはデフォルトで標準ライブラリに定義されているアイテムの中のいくつかを全てのプログラムのスコープに取り込む(**prelude**)
- 使いたい型がpreludeにない時は、`use`文で明示的にスコープに入れる (`std::io`はユーザとの入出力の処理に関しての機能を持つ)
- 変数を作る時は`let`文を使う
- 変数はデフォルトでは不変(immutable)になる。可変(mutable)にするには、変数名の前に`mut`をつける
- 等号記号(`=`)はRustに、変数を何かに束縛したいことを伝える効果がある
- `String::new`は、`String`型の新しいインスタンスを返す関数
- `String`は標準ライブラリにより提供される文字列型で、サイズが拡張なのうな、UTF-8でエンコードされたテキスト片
- `::`構文は、型に関しての関連関数であることを示す (`String::new`ならば、`new`は`String`に対して実装されているもの)
- `&`は引数が参照であることを示す、ここで参照もデフォルトでイミュータブルであり、`&mut var_name`のようにして書く必要がある点に注意
- `read_line`メソッドは、ユーザが入力した文字列に加え、同時に他の値も返している (この場合では`io::Result`)
- `Result`型は列挙型(enum)で、列挙子として`Ok`および`Err`を持つ
- `expect`メソッドを使用することで、`io::Result`インスタンスが`Err`の場合はプログラムをクラッシュさせ、`Ok`列挙子が保持する戻り値を取り出してその値だけを返してくれるようになる
- `println!`マクロのプレースホルダーは`{}`で書く (Pythonなら`{} + .format()`)
- Cargoはセマンティックバージョンを理解していて、`Cargo.toml`に外部クレートのバージョンを記述すると、そのバージョンと互換性のある公開APIを持つようになる
- `Cargo.lock`ファイルには依存関係のバージョンが記載されていて、これによりクレートのアップデートがあってもビルドに再現性を持たせられる
- クレートをアップグレードしたい場合は、cargo update`とする。`
- `match`式は複数のアームで構成され、各アームはマッチさせるパターンと、`match`に与えられた値がそのアームのパターンにマッチした時に実行されるコードいより構成される
- Rustでは変数のシャドウイングが許されていて、型の変換を行う際によく登場する
- 変数名の後に`:`をつけることで変数の型に注釈をつけることができる
- 無限ループの作成には`loop`キーワードを作成

### 3章: 一般的なプログラミングの概念
#### 3-1. 変数と可変性
- 定数を宣言したい時には、`let`キーワードの代わりに`const`キーワードを使用し、**必ず**型の注釈が必要
- 定数は定数式にしかセットできず、関数呼び出し結果や実行時に評価されるzたいをセットすることはできない
- 定数の命名規則は、すべて大文字で記述しアンダースコアで単語区切りする

#### 3-2. データ型
- Rustは静的型付け言語なので、複数の方が推論される可能性がある場合には型注釈を付けないとコンパイルエラーになる
- Rustでのスカラー型は、整数、浮動小数点数、論理値、文字の4つ
##### 整数型
| 大きさ | 符号付き | 符号なし |
|:----------:|:-----------:|:------------:|
| 8-bit      | i8          | u8           |
| 16-bit     | i16         | u16          |
| 32-bit     | i32         | u32          |
| 64-bit     | i64         | u64          |
| arch       | isize       | usize        |

- 整数リテラルは、接頭辞として16進数で`0x`、8進数で`0o`、2進数で`0b`、バイト(`u8`)で`b`があり、接尾辞として見た目の区切り記号として`_`をつけることができる

##### 浮動小数点型
- 演算速度と精度の兼ね合いから`f64`が基準系になっている
- 例えば、`let x = 2.0;`では、`f64`型が割り当てられる

##### 論理値型
- 型名は`bool`と指定し、`true`と`false`のいずれかの値を取る
- `if`式において主に用いられる

##### 文字型
- アルファベット型には`char`型が用意、シングルクオートで囲む
- ユニコードのスカラー値を表しており、asciiよりは遥かに多くの文字を表現可能(第8章で詳細)

##### 複合型
- 複合型を用いることで、複数の値を一つの方にまとめることができる
- 基本的な複合型としてはタプルと配列が存在
- タプルは丸括弧の中に`,`区切りの値リストを書くことで生成され、タプル内の値は異なる型でもよい
- タプルは1つの複合要素と考えられ、変数はタプル全体に束縛される
- 個々の値を取り出すには、パターンマッチングを使用して分解することができる(これを**分配**という)
- このほかに、アクセスしたい値の番号をピリオドに続けて書くことでタプルの要素に直接アクセスすることができる

##### 配列型
- Rustの配列は固定長で、スタックに確保される
- 可変長配列を被用したい場合はベクタ型を使用すればよい(第8章)
- 配列の型は角括弧の中に要素の方とセミコロン、配列の要素数を与える (例: `let a: [i32; 2] = [42,-42];`)
- `let a = [3; 5];`のように書くことで、各要素に同じ値をもつように初期化することができる (例では3という値を5つもつ配列)
- 配列要素へのアクセスは、`array[index]`のようにして行う
- 配列外参照を行うと実行時エラーとなり、この時Rustでは**パニック**するという

#### 3.3. 関数
- Rustの関数と変数の命名規則は、スネークケースにするのが慣例
- 関数定義は`fn`キーワードで始まり、関数がmain関数に対して上下どちらで定義されているかは関係なく動作する
- 関数シグニチャでは引数の型注釈が必須
- (review: 文は何らかの動作をして値を返さない命令、式は結果値に評価される)
- Rustでは変数の生成における値の代入は文なので、`let x = (let y = 6);`はコンパイルエラー
- この動作はC言語などとは異なる
- Rustにおいて式は終端にセミコロンを含まず、式の終端にセミコロンをつけてしまうとその処理は文になってしまう
- 戻り値のある関数では、戻り値の型を`->`の後に書いて宣言する
- 関数の戻り値は関数本体ブロックの最後の式の値と同義で、`return`キーワードを用いて早期リターンして値を指定することもできるが、多くの関数では最後の式を暗黙的に返すようにされる

#### 3.4. コメント
- Rustではコメントは2連スラッシュで始め、行の終わりまで続く
- コメントが複数行にまたがる場合は各行に`//`を含めつ必要がある

#### 3.5. 制御フロー
- `if`キーワードの後には条件式を続け、その直後に波括弧で囲んで配置する
- 条件式と紐づけられる一連のコードは、アームと呼ばれることがある
- `if`文の条件式は`bool`型でなければならない
- 複数の条件を扱いたい時は`else if`を使用するが、コードが読みにくくなった場合には`match`文を使用することを検討する
- `if`は**式**である (例: `let x = if condition { 5 } else { 6 }`)
- ただし、`if`と`else`アームでの型は互換性があるように選ばなければいけない
- 無限ループを書きたい時は`loop`キーワードを使用するとよい
- ループにはラベルがつけられ、`break`にこれを渡すことができる
- 条件付きの無限ループを呼びたい時は`while`ループを使用するとよい
- `for`は要素ごとにイテレーションする範囲`for`が可能 (例: `for elem in array {}`)で、安全性と簡潔さから推奨の書き方になっている
- `Range`型を使用することで高度な`for`の処理が可能になる (例: `rev`メソッド)

### 4章: 所有権を理解する
#### 4.1. 所有権とは？
- Rustの各値は、所有者と呼ばれる変数と対応し、いかなる場合も所有者は1つ
- 所有者がスコープから外れると、値は破棄される
- メモリの返還は`drop`関数により行われ、スコープを抜けると自動的にこれが呼び出される
- ヒープに確保された値を他の変数に代入しようとすると、ムーブ代入演算がよばれて所有権が移動、元々値を保持していた変数を参照するとコンパイルエラーになる
- deep copyが必要な場合には`clone`メソッドを使用する必要があるが、当然これは実行コストが比較的高い
- スタックにあるデータは、`Copy`トレイトに適合している場合、代入後も古い変数が使用可能になる
- 一般に単純なスカラー値の集合は`Copy`であり、メモリ確保されるリソースは`Copy`ではない (コピーのコストが大きくないため)

#### 4.2. 参照と借用
- 関数呼び出しの引数にしても所有権は移動してしまう
- これは値の所有権をもらう代わりに引数としてオブジェクトへの**参照**を取るようにすると解消される、この場合参照がスコープを抜けてもドロップされない
- 関数の引数に参照を取ることを、**借用**という
- 可変な参照を取る時は引数を渡す時点で、`&mut arg`のように書く、関数側も`(arg: &mut type)`のように宣言しておく
- 任意のタイミングで、一つの可変参照か不変な参照いくつでものどちらかを行うことができる
- 可変な参照は、特定のスコープで特定のデータに対しては1つしか持てない、これはデータレースを防ぐため
- また、不変な借用をしている間に可変な借用をすることはデータ競合の原因となり、認められない
- ダングリングポインタ: 他人に渡されてしまった可能性のあるメモリを指すポインタ、その箇所へのポインタを保持している間にメモリを解放してしまうことで発生
- Rustではコンパイラが参照がダングリング参照になることが絶対ないように保証されている
- 参照はつねに有効でなければならず、有効でなければコンパイルエラーとなる

#### 4.3. スライス型
- `[starting_index..ending_index]`と指定することでスライスが生成できる
- `starting_index`はスライスの最初の位置、`ending_index`はスライスの終端位置よりも1大きい値になる
- Rustの範囲記法`..`で、最初の番号(`0`)から始めたければ、二連ピリオドの前に値を置かないようにすればよく、末尾までのスライスを作るには最後の数字を書かなければよい (両方の値を省略すれば全体のスライスを得られる)
- 文字列スライスではマルチバイト文字の途中でスライスを生成しようとするとエラーになるので注意
- 文字列リテラルは、バイナリに埋め込まれた特定の位置を指すスライスとして書かれる、ゆえに不変な参照になっている (型: `&str`)
- 文字列以外のデータの型配列に対しても配列が取れて、この時の型は`&[type]`となる

### 5章: 構造体を使用して関係のあるデータを構造化する
#### 5.1. 構造体を定義し、インスタンス化する
- 各データ片に名前をつけることができ、データの順番に依存せずにインスタンスの値を指定できる
- インスタンスは、構造体名を記述し、`key: value`ペアを含む波括弧を付け加えることで生成され、フィールドは構造体で宣言した通りの順番に指定する必要はない
- 特定の値を得るにはドット記法が使える (例: `user1.email`)
- インスタンスの特定のフィールド値を変更するには構造体のインスタンス全体が可変である必要がある
- 仮引数名と構造体のフィールド名が同一の時はフィールド初期化省略記法が使える
- 構造体更新記法という記法があり、`..`を使用することで明示的にセットされていない残りのフィールドが与えられたインスタンスのフィールドと同じ値になるように指定できる
- 構造谷より追加の意味を含むが、フィールドに紐づけられた名前がなく、フィールドの方だけのタプルににた構造体を定義することが可能で、これを**タプル構造体**という (例: `struct Color(i32, i32, i32)`)
- 一切フィールドのない構造体を定義することもでき、これらは`()`ユニット型と似たような振る舞いをすることから、ユニット様構造体と呼ばれる
- ユニット様構造体は、ある方にトレイトを実装するが型そのものに保持させるデータはない場合に有効になる

#### 5.2. 構造体を使ったプログラム例
- `println!`マクロでは標準では波括弧は`Display`として知られる整形をするように指示される
- Rustでは`derive`注釈で使えるトレイトが多く提供されていて、独自の方に有用な振る舞いを追加することができる (例: `#[derive(Debug)]`)

#### 5.3. メソッド記法
- 構造体の文脈で関数を定義するには、`impl`ブロックを始める
- その後関数を`impl`の波括弧内に移動させて、シグニチャの引数に`self`を加える
- `impl`の文脈内に明示することで`self`の型が構造体のものであることをコンパイラに伝えられる
- メソッドは関数と全く同様に`self`の所有権を奪ったり、可変・不変で`self`を借用することができる
- メソッドの呼び出しはC, C++とは異なり、すべて`.`で行える、これはRustが自動参照および参照外しの機能を持つため
- `impl`ブロック内に`self`を引数に取らない関数を定義することができ、これを関連関数という
- 関連関数は構造体の新規インスタンスを返すコンストラクタによく使用される
- 呼び出しの際は`::`記法が使用される (例: `let sq = Rectangle::square(3);`)
- 各構造体に対して複数の`impl`ブロックを存在させることは許される、ユースケースは第10章で確認

### 6章: Enumとパターンマッチング
#### 6.1. Enumを定義する
- 取りうる値をすべて列挙でき、列挙型と呼ばれる
- `enum`に続いて型名を書いて列挙したい要素を書く、これを列挙子という
- enumの列挙子はその識別子の元に名前空間分けがされていて、`::`を使ってアクセスされる
- enumの列挙子に直接データを格納するような記法がある
##### 例
```Rust
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
- 上のように各列挙子に紐づけるデータ型と量は異なってもよく、これは構造体では表現不可能
- 構造体と同様にenumでも`impl`を使うことでメソッドが定義できる
- Rustにはnull機能がなく、値が存在するか不在かという概念をコード化するためには`Option`型が用いられる
```Rust
enum Option<T> {
	Some(T),
	None,
}
```
- `<T>`の記法はジェネリクス型引数のための記法
- `None`を使う場合はコンパイラに`Option<T>`の型が何かを伝えるために型注釈を入れる必要がある (例: `let absent_number : Option<i32> = None`)
- `Option<T>`を`T`であるように扱うには型変換が必要で、nullであるかどうかに関連して発生するありふれたバグの可能性を減らすことができる

#### 6.2. match制御フロー演算子
- `match`制御フロー演算子を用いると多様なパターンに対してアームを構成できる
- 値が適合する最初のパターンで紐づけられたコードブロックに移行して実行される
- アームのコードが短い場合は波括弧は使用しないことが一般的
- 全可能性を網羅していない場合にはコンパイルができず、包括性が担保される
- 全ての可能性を列挙したくない時には`_`プレースホルダーを使用することでどんな値にもマッチするものを考えることができる

#### 6.3. if letで簡潔な制御フロー
- マッチさせたいパターンが1つの時、`match`を使用して書くのは冗長
- `if let`という記法では、等号記号で区切られたパターンと式を取り、式が`match`に与えられて、パターンが最初のアームになった`match`と同じ動作をする (i.e. `match`の特定ケースのシンタックスシュガー)
- ただしこちらでは包括性のチェックはなく、ユースケースに合わせて使い分ける必要がある

### 7章: 肥大化していくプロジェクトをパッケージ、クレート、モジュールを使用して管理する
- コードのまとまりを保つための機能 (モジュールシステムと呼ばれる)として以下の様なものが存在する:
- **パッケージ:** クレートをビルドし、テストし、共有することができるCargoの機能
- **クレート:** ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
- **モジュール**と**use:** これの使用により、パスの構成、スコープ、公開するか否かの決定ができる
- **パス:** 要素 (例 構造体や関数やモジュール) に名前をつける方法

#### 7.1. パッケージとクレート
- クレートルート: Rustコンパイラの開始点となり、クレートのルートモジュールを作るソースファイル
- パッケージ: ある機能群を提供する1つ以上のクレートで、`Cargo.toml`という、これらのクレートをどの様にビルドするかを説明するファイルを持つ
- パッケージは0個か1個のライブラリクレートを持たなければならず、それ以上は認められない
- バイナリクレートはいくつ持っていてもよいが、少なくとも1つのクレートを持つ必要がある
- Cargoは`src/main.rs`がパッケージと同じ名前を持つバイナリクレートのクレートルートとなる
- 同様に、`src/lib.rs`がパッケージディレクトリに含まれている場合には、パッケージにはパッケージと同じ名前のライブラリクレートが含まれていて、`src/lib.rs`がそのクレートルートだと判断される
- パッケージに複数のバイナリクレートを持たせるには、`src/bin`ディレクトリ下にファイルを置く

#### 7.2. モジュールを定義して、スコープとプライバシーを制御する
- モジュールはクレート内のコードのグループ化、可読性と再利用性を上げるのに役立ち、要素のプライバシーの制御にも役立つ
- プライバシー: 要素がコードの外側で使える (public) のか、内部の実装の詳細であり外部では使えない (private) のかについての情報
- `mod`キーワードを書き、次にモジュールの名前を指定することで定義され、モジュールの中には他のモジュールを置くこともできる
- モジュールにはその他の要素の定義も置くことができ、構造体、enum、定数、トレイト、関数などが置ける

#### 7.3. モジュールツリーの要素を示すためのパス
- パスは、呼び出しのために必要なモジュールツリー内での一のことで、絶対パスと相対パスの二つの形を取ることができる
- 絶対パス: クレートの名前か`crate`という文字列を使うことで、クレートルートからスタートする
- 相対パス: `self`、`super`または今のモジュールないの識別子を使うことで、現在のモジュールからスタートする
- どちらの場合であっても、その後に一つ以上の識別子が`::`で区切られて続く
- Rustではあらゆる要素は標準では非公開になっており、公開するには`pub`キーワードをつける必要がある
- 親モジュールから始まる相対パスには、`super`を最初につけることで構成できる
- 構造体やenumも`pub`を使うことで公開の設定が可能
- 構造体の場合、各フィールドは非公開で、公開するにはフィールドにも`pub`キーワードが必要
- enumの場合は、enumを公開するとその変数は全て公開される

#### 7.4. useキーワードでパスをスコープに持ち込む
- `use`キーワードを用いるとパスをスコープに持ち込むことができ、以降はパス内の要素がローカルにあるかの様にして呼び出すことが可能になる
- 絶対パスをスコープに持ち込むには`crate`を、相対パスを持ち込むには`self`キーワードを使用する
- 関数をスコープに`use`で持ち込む場合には、親モジュールを持ち込むことが慣例的で、この様にすることでローカルで定義されたものでないことが明らかになる
- 一方、構造体やenumといった要素を`use`を持ち込む時はフルパスで書くのが慣例的 (例 `use std::collections::HashMap;`)
- 同じ名前の2つの要素を`use`でスコープに持ち込むのは禁止 (例 `use std::fmt::Result;` と `use std::io::Result;`)
- ただし`as`キーワードを使用するとこれを回避できる (例 `use std::io::Result as IoResult`)
- `use`で名前をスコープに持ち込んだ時、新しいスコープで使用できる名前は非公開
- これを公開するには`pub use`として組み合わせればよく、これを **再公開(re-exporting)** という
- 外部パッケージを使用するには`Cargo.toml`に追加する、こうすることで必要な依存をcrate.ioからダウンロードして自プロジェクトないで使える様にCargoに命令できる
- prefixが同じものはまとめてスコープに持ち込める (例 `use std::{cmp::Ordering, io};`)
- 片方がもう片方のサブパスである場合には`self`を用いる (例 `use std::io{self, Write};`)
- パスで公開されているすべての公開要素を持ち込みたい時は、glob演算子`*`をパスの後に続けて書く (例: `use std::collections::*`)

#### 7.5. モジュールを複数のファイルに分割する
- モジュール名の後にブロックではなくセミコロンを使うと、モジュールの中身をモジュールと同じ名前をした別のファイルから読み込む様に命令される
- さらにモジュール名のディレクトリを作成し、その中にモジュール内で定義されるモジュール名を持つファイルに定義を与えることでファイル分割できる

### 8章: 一般的なコレクション
#### 8.1. ベクタで値のリストを保持する
- ベクタ: 複数の値をヒープ上のメモリに隣り合わせに並べる
- `vec!`: 与えた値を保持する新しいベクタを生成するマクロ
- 要素の追加には、`push`メソッドを使用する、この時型には`mut`キーワードが必要 (削除は`pop`)
- `push`で要素を追加する場合には型がわかるので、型注釈は不要になる
- スコープを抜けるとベクタがドロップされ、その**中身もドロップされる**
- 要素の取得は`[]`メソッドまたは`get`メソッドを使用する
- `[]`を用いて配列外参照が起きた場合にはプログラムをパニックさせる
- `get`を用いて配列外を参照しようとすると`Option`型の`None`が返される
- ベクタの実装され方(*)から、要素への参照を保持しつつ可変参照を行おうとするとコンパイルエラーになる
- (*) 要素を隣合わせに確保できない時、一度今のメモリを解放して再確保する
- ベクタは同じ方の値しか保持有できないが、enumを使用することで異なる型の要素を保持することができる
- ただしベクタに保持する型が実行時に決定する場合はこの方法は有効でない、この場合はトレイトオブジェクト(17章)を使用する

#### 8.2. 文字列でUTF-8でエンコードされたテキストを保持する
- String型は標準ライブラリで提供される伸長可能、可変、所有権のあるUTF-8エンコードされた文字列型
- 標準ライブラリには`String`と`&str`の他にも文字列型が存在するが、`String`で終わるものは所有権のあるもの、`Str`で終わるものは所有権のないものを指している
- 文字列スライスからStringインスタンスを作るにはString::from()と、"literal".to_string()などの書き方がある
- Stringを伸ばす場合には文字列を末尾に加えられる`push_str`メソッド、1文字を引数として受け取る`push`メソッドなどが使用できる
- `push_str`メソッドでは引数の文字列の所有権は奪われない (`&str`として借用される)
- String同士の結合は`+`演算子や、`format!`マクロが使用できる
- `+`演算子では所有権のムーブが起こりうることに注意する (例: `let s = s1 + &s2`ならば、`s1`の所有権は`s`に移動している)
- UTF-8でエンコードされていることから文字のバイト数は必ずしも一定でなく、`String`への添え字アクセスはできない
- `String`の内部表現は`Vec<u8>`であり、厳密には`O(1)`でのアクセスを保証できないことも添え字アクセスできない理由の一つ
- したがって、文字列スライスを添え字を用いて取る際はクラッシュの可能性があり、慎重に使用する必要がある
- イテレーションを取る際は、`chars`メソッドを用いることで各要素へのアクセスができ、`bytes`メソッドを用いると各バイトを返す様になる

#### 8.3. キーとそれに紐づいた値をハッシュマップに格納する
- `HashMap<K, V>`により`K`型のキーと`V`型の値の対応関係を保持する
- `insert`メソッドを使用したり、タプルのベクタについて`collect`メソッドを使用するなどして要素を追加できる
- キーや値は挿入されるとハッシュマップに所有権が移動する
- 値の取得には`get`メソッドが使用でき、戻り値は`Option<&V>`となる
- 走査の際はタプルで取り出す (例: `for (key, value) in &scores`)
- キーと値の対応は一対一対応で、同じキーに対して複数回`insert`を読んだ場合は最後の値が保持される
- `entry`と呼ばれるAPIを使用することで、キーがなかった場合にのみ挿入したり、可変参照をとって昔の値に基づいて対応する値の更新などができる
- ハッシュ関数として標準のものとは別のものを指定することもできる

### 9章: エラー処理
#### 9.1. panic!で回復不能なエラー
- `panic!`マクロが実行されると、プログラムは失敗のメッセージを表示して、スタック位を巻き戻し掃除して終了する
- Cargo.tomlの`[profile]`欄に`panic = 'abort'`を追記すると、パニック時の処理を巻き戻しから以上終了する様に変更することができる
- バッファオーバーフロー攻撃の対策のため、ベクタでは範囲外アクセスが行われると`panic!`が呼ばれて実行が停止される
- バックトレースを見るには`RUST_BACKTRACE`環境変数をセットする

#### 9.2. Resultで回復可能なエラー
- `Result`型は以下の様に定義されている
```Rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```
- `T`は成功した時に`Ok`列挙子に含まれて返される値の型を、`E`は失敗した時に`Err`列挙子に含まれて返されるエラーの型を表すジェネリクス
- `Result`型は`match`式と相性が良い
- 例えば`File::open`を使用する亜愛には、`io::ErrorKind`値に基づいて、`Err`列挙子が返ってきた時の挙動を制御することができる
- マッチアームに条件式を足すことができ、これを**マッチガード**という
- パターンマッチングの際に変数に参照をバインドする際には、`ref`キーワードを使用する (18章)
- `match`の使用はしばしば冗長で、`Result`値が`Ok`ならばその中身を返す`unwrap`メソッドが利用できる
- `expect`を使用することで`Err`が返ってきた際のエラーメッセージを調整できる
- 関数内でエラー処理をせずに、呼び出し元にエラーを返す様に書くことを、エラーの**委譲**という
- `?`演算子を利用することでエラーの委譲を簡潔に実装できる
- `match`式と`?`演算子の違いとして、`?`演算子は標準ライブラリの`From`トレイトで定義される、エラーの型を変換する`from`関数を経由していることが挙げられる
- `?`演算子が使用できるのは戻り値が`Result`型の関数のみ、あくまで`match`式での`return Err(e)`の書き方と互換性を持つものであることに注意する

#### 9.3. panic!すべきかするまいか
- panicさせた方がいいケースとして、テストやプロトタイプコードなどが挙げられる
- また、プログラマから見て明らかに`Ok`が返る場合には、`unwrap`などを用いてpanicが発生しうるような書き方が認められる
