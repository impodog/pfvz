arg="$1"
filename=${arg%.*}
ffmpeg -i "$1" -f ogg -b:a 128k "$filename".ogg
rm "$1"
