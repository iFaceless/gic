#!/bin/bash

TMP_DIR=/var/tmp/install_gic
INSTALL_TO=/usr/local/bin

mkdir -p $TMP_DIR
cd $TMP_DIR

echo "download gic"
wget -q https://raw.githubusercontent.com/iFaceless/gic/master/bin/gic
chmod a+x gic

echo "install gic to dir $INSTALL_TO"
mv gic $INSTALL_TO
cd ~

rm -rf $TMP_DIR
gic -V
