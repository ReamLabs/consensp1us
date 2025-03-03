#!/bin/bash

INPUT_FILE=$1
OUTPUT_FILE=$(echo $INPUT_FILE | awk -F/ '{print $1 "/" "sorted_" $2}')

HEADER=$(head -n 2 "$INPUT_FILE")

echo "$HEADER" > "$OUTPUT_FILE"
tail -n +3 "$INPUT_FILE" | sort -t '|' -k1,1 -k2,2 >> "$OUTPUT_FILE"

rm $INPUT_FILE
