KERNEL = ./kernel/laranja-kernel
LOADER = ./bootloader/target/x86_64-unknown-uefi/release/laranja-loader.efi
BOOTIMAGE = LARANJA.img
TESTIMAGE = LARANJA-test.img

.PHONY: qemu
qemu: $(BOOTIMAGE)
	./qemu-run.sh

.PHONY: test
test: $(TESTIMAGE)
	./qemu-run.sh $(TESTIMAGE)

.PHONY: clean
clean: clean-loader clean-kernel
	rm -f $(BOOTIMAGE)


$(BOOTIMAGE): build $(KERNEL) $(LOADER)
	./make-image.sh

$(TESTIMAGE): build-test-kernel $(LOADER)
	./make-image.sh $(TESTIMAGE)

.PHONY: build-test-kernel
build-test-kernel:; cd kernel && cargo test --no-run --release

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
