#!/usr/bin/env sh

if [ "$#" -eq 0 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

echo s "$@" > ./input

cat ./output | sk | \
    awk '{print $NF}' | \
    xargs -r -I{} echo play {} > ./input