KERNEL = ./kernel/target/x86_64-unknown-none-mikankernel/release/laranja-kernel
LOADER = ./bootloader/target/x86_64-unknown-uefi/release/laranja-loader.efi
BOOTIMAGE = LARANJA.img

.PHONY: qemu
qemu: $(BOOTIMAGE)
	./qemu-run.sh

.PHONY: clean
clean: clean-loader clean-kernel
	rm -f $(BOOTIMAGE)

$(BOOTIMAGE): build $(KERNEL) $(LOADER)
	./make-image.sh

.PHONY: build
build: build-kernel build-loader

.PHONY: build-kernel
build-kernel:;	cd kernel && cargo build --release

.PHONY: build-loader
build-loader:;	cd bootloader && cargo build --release

.PHONY: clean-kernel
clean-kernel:; cd kernel && cargo clean

.PHONY: clean-loader
clean-loader:; cd bootloader && cargo clean
