# Laranja OS

An attempt to implement [MikanOS](https://github.com/uchan-nos/mikanos)(an educational OS) in Rust.
Mikan means mandarin in Japanese, and laranja means orange in Portuguese.

[『ゼロからのOS自作入門』](https://book.mynavi.jp/ec/products/detail/id=121220)を読みながらRustでかけるところを書いてみる。
IntelなLinuxを前提とするが、osbook_day03c-2以降はmacOSでも動く（はず）。

## problems with tags

tagはMikanOS/『ゼロからのOS自作入門』におおむね合わせているが、以下のような既知の問題がある。

* osbook_day03aは実際にはただしくうごいていない。kernel_mainが呼び出せていないため。
* osbook_day05f以前のタグでは、kernelをロードするallocate_poolの容量が誤っているため、環境によっては動かないかもしれない。[このコミット](ef641e9dfe18b0b6ec8df0e6ffd06c84764d8e60)で修正されている。

## prepare

Rust nightlyのほか、qemuなどが必要。『ゼロからのos自作入門』の環境設定ができていれば基本的にはOKのはず。
macOSでは、Homebrewのllvmをインストールし、llvmのbinにPATHが通っている必要がある。

## Build

`make`でbootloaderとkernelをビルド・QEMUで実行まで行う。

