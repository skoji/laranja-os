#!/bin/sh
qemu-system-x86_64 -monitor stdio -bios OVMF.fd -drive file=LARANJA.img,format=raw
