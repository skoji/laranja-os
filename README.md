# rust UEFI mikan

[『ゼロからのOS自作入門』](https://book.mynavi.jp/ec/products/detail/id=121220)を読みながらRustでかけるところを書いてみる。
intelなmacOSを前提にしている。

## prepare

Rust nightlyのほか、以下が必要

```
brew install dosfstools
brew install qemu
```

## Build

`bootloader`と`kernel`でそれぞれビルドする。

## QEMUで実行

```
./make-image.sh
./qemu-run.sh
```

