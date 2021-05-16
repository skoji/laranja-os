#!/bin/sh

if [ -z "$1" ];  then
   IMG_NAME=LARANJA.img
else
   IMG_NAME="$1"
fi
    
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
else
    sudo mount -o loop $IMG_NAME $MOUNT_POINT
fi

$SUDO ./write-object-to.sh $MOUNT_POINT

if [ `uname` = "Darwin" ]; then
    hdiutil detach $MOUNT_POINT
else
    sudo umount $MOUNT_POINT
fi
