#!/bin/sh
IMG_NAME=RMIKAN.img
MOUNT_POINT=./mnt

rm -f $IMG_NAME
qemu-img create -f raw $IMG_NAME 200M

mkfs.fat -n 'RMIKAN OS' -s 2 -f 2 -R 32 -F 32 $IMG_NAME

rm -rf $MOUNT_POINT
mkdir -p $MOUNT_POINT

if [ "`uname`" = "Darwin" ]; then
    hdiutil attach -mountpoint $MOUNT_POINT $IMG_NAME
    SUDO=''
else
    sudo mount -o loop $IMG_NAME $MOUNT_POINT
    SUDO=sudo
fi

$SUDO mkdir -p $MOUNT_POINT/EFI/BOOT
$SUDO cp ./bootloader/target/x86_64-unknown-uefi/release/rust-uefi-mikan.efi $MOUNT_POINT/EFI/BOOT/BOOTX64.EFI
$SUDO cp ./kernel/target/x86_64-unkown-none-mikankernel/release/rmikan-kernel $MOUNT_POINT/rmikan-kernel

if [ `uname` = "Darwin" ]; then
    hdiutil detach $MOUNT_POINT
else
    sudo umount $MOUNT_POINT
fi
