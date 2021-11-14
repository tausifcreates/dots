#!/usr/bin/env bash

snixembed > /dev/null 2>&1 &

killall mate-power-manager > /dev/null 2>&1; mate-power-manager > /dev/null 2>&1 &

sleep 0.5

killall iwgtk > /dev/null 2>&1; iwgtk -i &

sleep 0.5

killall mictray > /dev/null 2>&1; mictray &

sleep 0.5

killall volumeicon > /dev/null 2>&1; volumeicon &
