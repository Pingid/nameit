cargo leptos build --release -vv
cd target/pkg
rm *.br
ls *.js *.wasm *.css | xargs brotli