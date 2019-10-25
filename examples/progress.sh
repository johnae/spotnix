#!/usr/bin/env sh

## requires jq

EVENTS=${SPOTNIX_EVENT:-./event}

while true; do
    LINE="$(jq -r '.PlaybackStatus | .progress = (.progress_ms / .duration_ms) * 100 | "Track: \(.track) - Album: \(.album) - Artists: \(.artists | join(", ")) - \(.progress | floor)%"' < "$SPOTNIX_EVENT")"
    echo -ne "$LINE\033[0K\r"
    sleep 0.1
done