#!/bin/env zsh

if [ -z "$1" ]
then
    for DAY in {1..25}
    do
        DAY=${(l:2::0:)DAY}
        time cargo run --release -- $DAY < inputs/day$DAY.txt
    done
else
    DAY=${(l:2::0:)1}
    time cargo run --release -- $DAY < inputs/day$DAY.txt
fi
