#!/bin/bash

if [ "$(hostname)" = "ubupi4a_" ] ; then
    exec l2cat connect -r E4:5F:01:49:DD:D8 240
else
    exec l2cat connect -r DC:A6:32:F9:C9:F8 240
fi
