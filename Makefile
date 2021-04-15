KERNEL = ./kernel/target/x86_64-unknown-none-mikankernel/release/laranja-kernel
LOADER = ./bootloader/target/x86_64-unknown-uefi/release/laranja-loader.efi
BOOTIMAGE = LARANJA.img

.PHONY: qemu
qemu: $(BOOTIMAGE)
	./qemu-run.sh

.PHONY: clean
clean:; rm -rf kernel/target/*; rm -rf bootloader/target/*; rm -f $(BOOTIMAGE)

$(BOOTIMAGE): build-kernel build-loader $(KERNEL) $(LOADER)
	./make-image.sh

.PHONY: build-kernel
build-kernel:;	pushd kernel && cargo build --release && popd
.PHONY: build-loader
build-loader:;	pushd bootloader && cargo build --release && popd

