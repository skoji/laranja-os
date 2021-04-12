# rust UEFI mikan

[『ゼロからのOS自作入門』](https://book.mynavi.jp/ec/products/detail/id=121220)を読みながらRustでかけるところを書いてみる。

<s>intelなmacOSを前提にしている。</s>
IntelなLinuxを前提とする。

## tags

* osbook_day03aは実際にはただしくうごいていない。kernel_mainが呼び出せていないため。

## prepare

Rust nightlyのほか、qemuなどが必要。『ゼロからのos自作入門』の環境設定ができていれば基本的にはOKのはず。

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

