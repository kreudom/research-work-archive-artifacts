qemu-system-x86_64  -enable-kvm -hda deb.img -cdrom ubuntu-24.04.1-live-server-amd64.iso -m 65536 -display curses -nographic -smp 32,sockets=1,cores=16,threads=2,maxcpus=32
