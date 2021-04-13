KERNEL = ./kernel/target/x86_64-unknown-none-mikankernel/release/rmikan-kernel
LOADER = ./bootloader/target/x86_64-unknown-uefi/release/rust-uefi-mikan.efi
BOOTIMAGE = RMIKAN.img

.PHONY: qemu
qemu: $(BOOTIMAGE)
	./qemu-run.sh

.PHONY: clean
clean:; 

$(BOOTIMAGE): build-kernel build-loader $(KERNEL) $(LOADER)
	./make-image.sh

.PHONY: kernel
build-kernel:;	./kernel/build.sh
.PHONY: loader
build-loader:;	./bootloader/build.sh

