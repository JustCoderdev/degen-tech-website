set -e

outpath="./${1:-build}"

echo "Copying static files to '${outpath}'"

mkdir -p "$outpath"
cp -r 'assets' "$outpath"
ln -fs "assets/robots.txt" "$outpath/robots.txt"

echo "Compiling and running generator.rs"
rustc generator.rs
./generator
