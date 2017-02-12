#!/usr/bin/env bash
echo "Build application"
xargo build

echo "Copy binary"
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/debug/due-blink target/blink.bin

echo "Touch programming port ..."
stty -F "/dev/ttyACM0" raw ispeed 1200 ospeed 1200 cs8 -cstopb ignpar eol 255 eof 255
printf "\x00" > "/dev/ttyACM0"
echo "Waiting before uploading ..."
sleep 1
echo "Uploading ..."
/home/klangner/bin/bossac/bossac --write --verify --boot -R target/blink.bin