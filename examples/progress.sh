#!/usr/bin/env sh

while true; do
    PLAYING="$(cat ./event)"
    PERCENTAGE="$(echo "$PLAYING" | awk -F'#' '{print $5"/"$6}' | sed -e 's|duration_ms:||g' -e 's|progress_ms:||g' | xargs -r -I{} echo 'scale=2; ({}) * 100' | bc)"
    TRACK="$(echo "$PLAYING" | awk -F'#' '{print $2}' | sed 's|track:||g')"
    ALBUM="$(echo "$PLAYING" | awk -F'#' '{print $3}' | sed 's|album:||g')"
    ARTISTS="$(echo "$PLAYING" | awk -F'#' '{print $4}' | sed 's|artists:||g')"
    echo -ne "$TRACK | $ARTISTS | $ALBUM - $PERCENTAGE %\033[0K\r"
done