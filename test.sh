#!/usr/bin/env sh

RUSTFLAGS='-C target-cpu=native' cargo build --release

pv -N csv test_data/sentences.csv | ./target/release/srg '\. Dimes ' csv 1 | wc -l
pv -N json test_data/sentences.json | ./target/release/srg '\. Dimes ' json text | wc -l
pv -N prefixed test_data/sentences.txt | ./target/release/srg '\. Dimes ' prefixed | wc -l
pv -N tar test_data/sentences.tar | ./target/release/srg '\. Dimes ' tar | tar -tvf - | wc -l
