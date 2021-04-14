#!/bin/sh
IMG_NAME=LARANJA.img
MOUNT_POINT=./mnt

if [ "`uname`" = "Darwin" ]; then
    MKFS_FAT=/usr/local/opt/dosfstools/sbin/mkfs.fat
    SUDO=''
else
    MKFS_FAT=mkfs.fat
    SUDO=sudo
fi

rm -f $IMG_NAME
qemu-img create -f raw $IMG_NAME 200M

$MKFS_FAT -n 'LARANJA OS' -s 2 -f 2 -R 32 -F 32 $IMG_NAME

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
$SUDO cp ./bootloader/target/x86_64-unknown-uefi/release/laranja-loader.efi $MOUNT_POINT/EFI/BOOT/BOOTX64.EFI
$SUDO cp ./kernel/target/x86_64-unknown-none-mikankernel/release/laranja-kernel $MOUNT_POINT/rmikan-kernel

if [ `uname` = "Darwin" ]; then
    hdiutil detach $MOUNT_POINT
else
    sudo umount $MOUNT_POINT
fi
