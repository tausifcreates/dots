#!/usr/bin/env bash

snixembed > /dev/null 2>&1 &

killall iwgtk > /dev/null 2>&1; iwgtk -i &

killall pasystray > /dev/null 2>&1; pasystray &

killall mictray > /dev/null 2>&1; mictray &

mate-power-manager > /dev/null 2>&1 &
