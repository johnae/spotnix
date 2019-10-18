#!/usr/bin/env sh

DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)"

if [ "$#" -eq 0 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

echo sap "$@" > "$DIR"/input

cat "$DIR"/output | sk | \
    awk '{print $NF}' | \
    xargs -r -I{} echo play {} > "$DIR"/input