#!/bin/bash

files=(
"/home/obey/Pictures/wallpaper/anime_city_futuristic.png"
"/home/obey/Pictures/wallpaper/anime_dragon_man_light.jpg"
"/home/obey/Pictures/wallpaper/cyber-girl-light.png"
"/home/obey/Pictures/wallpaper/lake_sunset_landscape.png"
"/home/obey/Pictures/wallpaper/logo_nasa_japanese.png"
)

for filename in ${files[@]}; do
  rwal $filename -t -c 16 -b wal
done

for filename in ${files[@]}; do
  rwal $filename -t -c 16 -b median-cut
done
