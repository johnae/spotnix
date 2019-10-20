#!/usr/bin/env sh

if [ ! "$#" -eq 0 ]; then
    echo "Illegal number of parameters (takes 0 parameters)"
    exit 1
fi

echo ld "$@" > ./input

cat ./output | sk | \
    awk '{print $NF}' | \
    xargs -r -I{} echo device {} > ./input