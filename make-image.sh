#!/bin/sh
IMG_NAME=RMIKAN.img
MOUNT_POINT=./mnt

rm -f $IMG_NAME
qemu-img create -f raw $IMG_NAME 200M
/usr/local/opt/dosfstools/sbin/mkfs.fat -n 'RMIKAN OS' -s 2 -f 2 -R 32 -F 32 $IMG_NAME
rm -rf $MOUNT_POINT
hdiutil attach -mountpoint $MOUNT_POINT $IMG_NAME
mkdir -p $MOUNT_POINT/EFI/BOOT
cp ./bootloader/target/x86_64-unknown-uefi/release/rust-uefi-mikan.efi $MOUNT_POINT/EFI/BOOT/BOOTX64.EFI
cp .//kernel/target/x86_64-unknown-linux-gnu/release/rmikan-kernel $MOUNT_POINT/rmikan-kernel
hdiutil detach $MOUNT_POINT
