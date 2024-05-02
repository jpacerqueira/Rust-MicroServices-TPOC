##!/usr/bin/env bash
#
## Install libc
#
wget -c https://ftp.gnu.org/gnu/glibc/glibc-2.29.tar.gz
tar -zxvf glibc-2.29.tar.gz
mkdir glibc-2.29/build
cd glibc-2.29/build
../configure --prefix=/opt/glibc
make
make install
#
## If fails run
sudo apt-get install build-essential
#
