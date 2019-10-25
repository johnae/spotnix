#!/usr/bin/env sh

if [ "$#" -eq 0 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

echo sar "$@" > ./input

sk < ./output | \
    awk '{print $NF}' | \
    xargs -r -I{} echo play {} > ./input