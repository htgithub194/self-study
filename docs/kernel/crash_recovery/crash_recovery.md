# Crash recovery and debugging 

This artical introduces a way that Linux Kernel can quickly recover from a crash and also support capturing the contents of system memory for further debugging
	

## The big picture

[Product Kernel] --crash--> [Crash Kernel] --> [Save crash dump] --> [Reboot/Analyze]


In the very first boot, reserve a small memory in RAM by using boot option, e.g: `crashkernel=200M`

When product kernel goes live, using `kexec` to load a smaller kernel + initrd, let's call it capture-kernel, on to the reserve region above.

In case our system (with product kernel) suffers a crash, the capture-kernel is automatically booted up.

Notice that the capture kernel should only live inside the resevered memory if we want to do further debugging.

Using kdump captures the whole system memory to a dumpfile. The dumpfile then can be stored somewhere, on persistent storage, for further debugging.




## Kexec

* kexec allows you to load and boot a new kernel directly from a running system without going through the BIOS/firmware or reboot cycle.

* This is especially useful for:
    * Faster reboots (skip hardware init)
    * Crash recovery (boot into a crash kernel to capture memory dumps)

## Kdump

* Dumps the main kernel's memory into `/var/crash/vmcore` using `makedumpfile` or `kdumpctl`

## Crash

* Inspect the `vmcore`:

    ```shell
    crash /usr/lib/debug/vmlinux-$(uname -r) /var/crash/vmcore
    ```

## Practice on Alpine

### create custom image 

* Create ram based image which contains kexec

    * Ref: [Minirootfs Alpine & internet connection](../mini_alpine/mini_alpine.md)

```shell
# fetch minirootfs
wget https://dl-cdn.alpinelinux.org/alpine/v3.22/releases/x86_64/alpine-minirootfs-3.22.0-x86_64.tar.gz

# extracting
mkdir rootfs
tar -xzf alpine-minirootfs-3.22.0-x86_64.tar.gz -C rootfs

# bind with host for further chroot usage
cd rootfs
mount --bind /proc proc
mount --bind /dev dev
mount --bind /sys sys

# add DNS
echo "nameserver 8.8.8.8" > etc/resolv.conf

# change root
chroot . /bin/sh

# add feature: 
apk add alpine-base linux-virt mkinitfs

# keep terminal in chroot for further setup

```

### install kexec

```shell
# add repo. notice the http (not https)
echo "http://dl-cdn.alpinelinux.org/alpine/v3.22/main" >> /etc/apk/repositories
echo "http://dl-cdn.alpinelinux.org/alpine/v3.22/community" >> /etc/apk/repositories
apk update

# install kexec
apk add kexec-tools
```

### prepare capture image

* Store whatever capture kernel+initrd as you want.

* Here, I use the same product `vmlinuz-virt` + `initramfs-virt`, located at `boot`, as capture image

```shell
/ # cd boot/
/boot # tree
.
├── System.map-6.12.51-0-virt
├── boot -> .
├── config-6.12.51-0-virt
├── initramfs-virt
└── vmlinuz-virt
```

### prepare mkinitfs features

```shell
# add kexec to mkinitfs
cat > /etc/mkinitfs/features.d/kexec.files <<'EOF'
/usr/sbin/kexec
EOF

# add capture_kernel
cat > /etc/mkinitfs/features.d/capture_kernel.files <<'EOF'
/boot/
EOF

## some tweaks
# add dns
echo "/etc/resolv.conf" >> /etc/mkinitfs/features.d/base.files
# add apk update
echo "/etc/apk/world" >> /etc/mkinitfs/features.d/base.files
echo "/var/lib/" >> /etc/mkinitfs/features.d/base.files
echo "/lib/apk/db" >> /etc/mkinitfs/features.d/base.files
##

```

### make initfs

```shell
mkinitfs -F "base virtio kexec capture_kernel" -k 6.12.51-0-virt
# ==> initramfs: creating /boot/initramfs-virt for 6.12.51-0-virt
```

### boot up the new image

* Add boot option `crashkernel` to Qemu command

* Notice that we need to enable kexec using boot option `kexec_load_disabled` as Alpine disable this feature by default.

```shell
# Qemu
sudo qemu-system-x86_64 \
-m 512 \
-kernel vmlinuz-virt \
-initrd initramfs-virt \
-append "console=ttyS0 init=/init crashkernel=128M kexec_load_disabled=0" \
-nographic

# check inside Qemu node
~ # uname -r
6.12.51-0-virt
~ # ls /boot/
System.map-6.12.51-0-virt  initramfs-virt
config-6.12.51-0-virt      vmlinuz-virt
~ # which kexec
/usr/sbin/kexec
```

* Note: in case facing apk error, try to create dir `/var/lib`

```shell
~ # apk update
ERROR: Failed to open apk database: No such file or directory

~ # mkdir /var/lib -p
~ # apk update

OK: 49 distinct packages available
~ # 
```

### manualy switch to another kernel

* Use option `kexec -l` to load capture image

* Use `kexec -e` to trigger the switching

```shell
# use 
kexec -l /boot/vmlinuz-virt \
  --initrd=/boot/initramfs-virt \
  --command-line="init=/init console=ttyS0"
```

* Note: the `/proc/iomem` has changed after switching

| Memory Map 1 (Old)                      | Memory Map 2 (New)                      |
|----------------------------------------|----------------------------------------|
| `00100000-1ffdffff : System RAM`       | `00100000-1ffdffff : System RAM`       |
| `0b000000-0bdfffff : Kernel code`      | `1d000000-1ddfffff : Kernel code`      |
| `0be00000-0c703fff : Kernel rodata`    | `1de00000-1e703fff : Kernel rodata`    |
| `0c800000-0c9f247f : Kernel data`      | `1e800000-1e9f247f : Kernel data`      |
| `0d0bb000-0d1fffff : Kernel bss`       | `1f0bb000-1f1fffff : Kernel bss`       |
| `16000000-1dffffff : Crash kernel`     | *(none)*                               |

### Automatically switch when crashing

* Use `kexec -p` as `p` stands for panic.


```shell
kexec -p /boot/vmlinuz-virt \
  --initrd=/boot/initramfs-virt \
  --command-line="init=/init console=ttyS0 maxcpus=1"
```

* By default, alpine disable `PROC_KCORE`, it means we must re-build the alpine's kernel with this option enabled. So, I decided to end this artical here :(

```shell
/boot # cat config-6.12.51-0-virt  | grep -i kcore
# CONFIG_PROC_KCORE is not set
```