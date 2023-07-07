#!/bin/bash

set -e

build-rs() {
    make build-connectd-rs
}

build-go() {
    make build-connectd-go
}

call-rs() {
    busctl --user call \
        com.github.suse.ConnectdRs \
        /com/github/suse/ConnectdRs \
        com.github.suse.ConnectdRs \
        Version \
        b $1
}

call-go() {
    busctl --user call \
        com.github.suse.ConnectdGo \
        /com/github/suse/ConnectdGo \
        com.github.suse.ConnectdGo \
        Version \
        b $1
}

LANGUAGE=$1
FULL_VERSION=$2

if [ -z "$LANGUAGE" ]; then
    read -p "Select language (go/rs): " LANGUAGE
fi

if [ -z "$FULL_VERSION" ]; then
    read -p "Select fetch full version (y/n): " FULL_VERSION
fi

if [ "$LANGUAGE" == "rs" ]; then
    BUILD=build-rs
    RUN_DBUS=./connectd-rs/target/debug/connectd-rs
    CALL=call-rs
elif [ "$LANGUAGE" == "go" ]; then
    BUILD=build-go
    RUN_DBUS=./out/connectd-go
    CALL=call-go
else
    echo Language needs to either be go or rs
    exit 1
fi

if [ "$FULL_VERSION" == "y" ]; then
    FULL_VERSION=yes
elif [ "$FULL_VERSION" == "n" ]; then
    FULL_VERSION=no
else
    echo FULL_VERSION needs to be y or n
    exit 1
fi



$BUILD
$RUN_DBUS &
BUS_ID=$!
sleep 1
$CALL $FULL_VERSION
sleep 1
kill $BUS_ID
