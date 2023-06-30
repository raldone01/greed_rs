#!/bin/fish

set -l text $argv[1]

for font in /usr/share/figlet/*
    printf "\n$font\n"
    set -l font (basename $font)
    toilet -f $font -t $text
end
