#!/usr/bin/env sh

BASEDIR=$(dirname "$0")
TEMPLATE="{{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }}  {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }}  {{ Sentence }} {{ Sentence }} {{ Sentence }} {{ Sentence }}"

echo "$TEMPLATE" | fakedata -l 100000 | jq -Rc '{text: ., id: input_line_number}' > "$BASEDIR"/sentences.json
jq -r '[.id, .text] | @csv' "$BASEDIR"/sentences.json > "$BASEDIR"/sentences.csv
jq -r '[.id, (.text | length) .text] | @tsv' "$BASEDIR"/sentences.json > "$BASEDIR"/sentences.csv
python3 test_data/prefixed.py < "$BASEDIR"/sentences.json > "$BASEDIR"/sentences.txt
python3 test_data/tar.py < "$BASEDIR"/sentences.json > "$BASEDIR"/sentences.tar