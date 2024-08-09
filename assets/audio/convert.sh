filename=$(basename "$1")
dir=$(dirname "$1")
ffmpeg -i "$1" -f ogg -b:a 128k "$dir"/"$filename".ogg
rm "$1"
