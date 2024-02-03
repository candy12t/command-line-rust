# command-line-rust

## 1

--binで実行するバイナリターゲットを指定できる

```bash
cargo run --bin hoge
```

シングルスレッドでテストを実行

```bash
cargo test -- --test-threads=1
```

## 2

main関数のように戻り値の型を省略している場合は、[**ユニット型**](https://doc.rust-lang.org/std/primitive.unit.html)を返している
ユニット型は空の値のようなもので、() と表現される
ユニット型はnullポインタや未定義値とは異なる

カレントディレクトリのサイズ確認
```bash
du -shc .
```

targetディレクトリを削除
```bash
cargo clean
```

Noneに対して Option::unwrap を呼ぶと、パニックになる
値がSomeであることが確実な場合のみ、unwrapを呼ぶ

テスト名にhogeと含まれるテストを実行
```bash
cargo test hoge
```

Vec: 伸長可能な配列
Box: データはヒープ領域に格納され、そのデータのポインタがスタックに格納される
