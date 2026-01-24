#!/bin/sh
if [[ ${#} -lt 3 ]]; then
  echo "usage: pac.sh <file> <trigger> <keyword>" >&2
  exit 1
fi

file="$1"
trigger="$2"
keyword="$3"

awk -v trg="$trigger" -v kw="$keyword" '
{
    if ($0 ~ trg) {
        if ($0 ~ kw) {
            print_flag = 1
            print $0
        } else {
            print_flag = 0
        }
    } else if (print_flag) {
        print $0
    }
}
' "$file"
