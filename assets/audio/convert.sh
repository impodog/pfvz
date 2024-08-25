for i in "$@"; do
	arg="$i"
	filename=${arg%.*}
	ffmpeg -i "$i" -f ogg -b:a 128k "$filename".ogg
	rm "$i"
done
