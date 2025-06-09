# Mini alpine

* Goal 1: Create & boot a pair of vmlinuz+initrd from alpine-minirootfs
    * Small size image
    ```shell
    -rw-------  1 root root 5.6M Jun  8 21:24 initramfs-virt
    -rw-r--r--  1 root root  12M Jun  8 07:09 vmlinuz-virt
    ```

    * Run in RAM only:
    ```shell
    qemu-system-x86_64 \
    -m 512M \
    -kernel vmlinuz-virt \
    -initrd initramfs-virt \
    -netdev tap,id=net0,ifname=tap0,script=no,downscript=no \
    -device virtio-net-pci,netdev=net0 \
    -append "console=ttyS0 init=/init" \
    -enable-kvm \
    -nographic
    ```

* Goad 2: provide internet connection via NAT
    
    ![internet](images/networks.dio.svg)


### Goal 1

* Download minirootfs from alpine homepage

    ```shell
    #
    wget https://dl-cdn.alpinelinux.org/alpine/v3.22/releases/x86_64/alpine-minirootfs-3.22.0-x86_64.tar.gz
    #
    mkdir rootfs
    #
    tar -xzf alpine-minirootfs-3.22.0-x86_64.tar.gz -C rootfs
    ```

* chroot to rootfs

    ```shell
    # bind host -> rootfs
    cd rootfs
    mount --bind /proc proc
    mount --bind /dev dev
    mount --bind /sys sys

    # provide DNS for rootfs
    echo "nameserver 8.8.8.8" > etc/resolv.conf

    # chroot to rootfs
    chroot . /bin/sh
    ```

* inside chroot, update package

    ```shell
    apk add alpine-base linux-virt mkinitfs
    ```

    * Note1. Change repository by:
        ```shell
        echo "http://dl-cdn.alpinelinux.org/alpine/v3.22/main" >> /etc/apk/repositories
        ```

    * Note 2. `alpine-base` provide:
        busybox – provides standard UNIX tools (sh, ls, mount, ifconfig, etc.)

        alpine-baselayout – sets up filesystem hierarchy (/etc, /var, /run, etc.)

        alpine-conf – contains init scripts, system config (/etc/inittab, setup-* scripts)

        libcrypt – password hashing

        libgcc – runtime library for GCC-built programs

        musl – the C standard library

        openrc – the default init system in Alpine

        apk-tools – the Alpine package manager (apk)


    * Note3. `linux-virt` is suitable with QEMU. 
        * This package provide:
            * /boot/vmlinuz-virt
            * /lib/module/6.12.31-0-virt
        * `linux-lts` for normal hardware

    * Note 4. Install some usefull package basedon your need:

        ```shell
        apk add iproute2
        ```

* Make initrd

    ```shell
     mkinitfs -F "base,virtio" -k 6.12.31-0-virt
    ==> initramfs: creating /boot/initramfs-virt for 6.12.31-0-virt
    ```

    * Node 0. `base virtio` is features

        * by default, mkinitfs builds features inside `/etc/mkinitfs/mkinitfs.conf`,
        * but, it not work in my env, so I specify it in the call to `mkinitfs`
        * Or: `mkinitfs -c /path/to/conf -F "features" -k <version>`

    * Note 1. `/etc/mkinitfs/mkinitfs.conf` contain features for initrd

        ```shell
        # default configure is:
        #    features="ata base cdrom ext4 keymap kms mmc nvme raid scsi usb virtio"
        #
        
        #
        # custom mkinitfs conf:
        #
        # Specify features to include
        features="base,virtio"

        # Kernel modules path (optional, defaults to /lib/modules/<version>)
        modloop=/lib/modules/6.12.31-0-virt

        # Additional files (optional, if not using feature.d/*.files)
        add_files="/sbin/openrc-init /bin/sh"
        ```

* QEMU boot

    ```shell
    qemu-system-x86_64 \
    -m 512 \
    -kernel vmlinuz-virt \
    -initrd initramfs-virt \
    -append "console=ttyS0 init=/init" \
    -nographic
    ```


### Goal 2

* Create bridge `br0` and `tap0`

    ```shell
    sudo ip link add name br0 type bridge
    sudo ip addr add 192.168.100.1/24 dev br0
    sudo ip link set br0 up

    # create tap device
    sudo ip tuntap add dev tap0 mode tap user $USER
    sudo ip link set tap0 master br0
    sudo ip link set tap0 up
    ```

* route between `eth0` -> `br0`:

    ```shell
    # alow traffic from br0 -> eth0 -> internet
    iptables -A FORWARD -i br0 -o eth0 -j ACCEPT

    # alow traffic back from eth0 -> br0
    iptables -A FORWARD -i eth0 -o br0 -m state --state ESTABLISHED,RELATED -j ACCEPT

    # NAT
     sudo iptables -t nat -A POSTROUTING -s 192.168.100.0/24 -o eth0 -j MASQUERADE
    ```

* boot QEMU with

    ```shell
    qemu-system-x86_64 \
    -m 512M \
    -kernel vmlinuz-virt \
    -initrd initramfs-virt \
    -netdev tap,id=net0,ifname=tap0,script=no,downscript=no \
    -device virtio-net-pci,netdev=net0 \
    -append "console=ttyS0 init=/init" \
    -enable-kvm \
    -nographic
    ```

    ```shell
    # config ip
    ip a add 192.168.100.200/24 dev eth0
    ip link set eth0 up

    # config route
    ip route add default via 192.168.100.1 dev eth0

    # DNS
    echo "nameserver 8.8.8.8" > /etc/resolv.conf
    ```

    * Note 0: use `virtio-net-pci` device because Alpine has virtio driver
        ```shell
        lsmod | grep vir
        virtio_net            118784  0 
        net_failover           24576  1 virtio_net
        ```

    * Note 1:
        ```shell
        # route
        ip route
        default via 192.168.100.1 dev eth0 
        192.168.100.0/24 dev eth0 scope link  src 192.168.100.200 

        # ip addr
        ip a show eth0
        2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state UP qlen 1000
            link/ether 52:54:00:12:34:56 brd ff:ff:ff:ff:ff:ff
            inet 192.168.100.200/24 scope global eth0
            valid_lft forever preferred_lft forever
            inet6 fe80::5054:ff:fe12:3456/64 scope link 
            valid_lft forever preferred_lft forever
        
        # ping local
        ping -c 3 192.168.100.1
        PING 192.168.100.1 (192.168.100.1): 56 data bytes
        64 bytes from 192.168.100.1: seq=0 ttl=64 time=0.203 ms
        64 bytes from 192.168.100.1: seq=1 ttl=64 time=0.198 ms
        64 bytes from 192.168.100.1: seq=2 ttl=64 time=0.220 ms

        --- 192.168.100.1 ping statistics ---
        3 packets transmitted, 3 packets received, 0% packet loss
        round-trip min/avg/max = 0.198/0.207/0.220 ms
    
        # ping google
        ping -c 3 google.com
        PING google.com (142.250.198.206): 56 data bytes
        64 bytes from 142.250.198.206: seq=0 ttl=114 time=51.240 ms
        64 bytes from 142.250.198.206: seq=1 ttl=114 time=51.227 ms
        64 bytes from 142.250.198.206: seq=2 ttl=114 time=51.228 ms

        --- google.com ping statistics ---
        3 packets transmitted, 3 packets received, 0% packet loss
        round-trip min/avg/max = 51.227/51.231/51.240 ms
        ```