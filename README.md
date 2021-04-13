# Laranja OS

Attemnt to build Rust port of [MikanOS](https://github.com/uchan-nos/mikanos)(an educational OS). 
Mikan means mandarin in Japanese, and laranja means orange in Portuguese.

[『ゼロからのOS自作入門』](https://book.mynavi.jp/ec/products/detail/id=121220)を読みながらRustでかけるところを書いてみる。
IntelなLinuxを前提とするが、osbook_day03c-2以降はmacOSでも動く（はず）。

## tags

* osbook_day03aは実際にはただしくうごいていない。kernel_mainが呼び出せていないため。

## prepare

Rust nightlyのほか、qemuなどが必要。『ゼロからのos自作入門』の環境設定ができていれば基本的にはOKのはず。
macOSでは、Homebrewのllvmをインストールし、llvmのbinにPATHが通っている必要がある。

## Build

`bootloader`と`kernel`でそれぞれビルドする。

```
./bootloade/build.sh
./kernel/build.sh
```

## QEMUで実行

```
./make-image.sh
./qemu-run.sh
```

