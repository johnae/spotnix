#!/usr/bin/env sh

while true; do
    PLAYING="$(cat ./event)"
    PERCENTAGE="$(echo "$PLAYING" | awk -F'#' '{print $5"/"$6}' | sed -e 's|duration_ms:||g' -e 's|progress_ms:||g' | xargs -r -I{} echo 'scale=2; ({}) * 100' | bc)"
    TRACK="$(echo "$PLAYING" | awk -F'#' '{print $2}' | sed -e 's|track:||g' -e 's/[[:space:]]*$//')"
    ALBUM="$(echo "$PLAYING" | awk -F'#' '{print $3}' | sed -e 's|album:||g' -e 's/[[:space:]]*$//')"
    ARTISTS="$(echo "$PLAYING" | awk -F'#' '{print $4}' | sed -e 's|artists:||g' -e 's/[[:space:]]*$//')"
    echo -ne "$TRACK | Artist: $ARTISTS | Album: $ALBUM - $PERCENTAGE %\033[0K\r"
done