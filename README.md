# rust UEFI mikan

[『ゼロからのOS自作入門』](https://book.mynavi.jp/ec/products/detail/id=121220)を読みながらRustでかけるところを書いてみる。

## Build

```
 cargo +nightly build -Zbuild-std=core,alloc --target  x86_64-unknown-uefi --release
```

`target/x86_64-unknown-uefi/release/rust-uefi.efi`ができるので、これをUSBメモリの`/EFI/BOOT/BOOTX64.EFI`にコピーする。



