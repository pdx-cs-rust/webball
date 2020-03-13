# webball — bouncing ball demo in Javascript canvas
Bart Massey 2020-03-12

This code, derived from the `wasm-bindgen` HTML Canvas demo,
bounces a ball in a browser window.

## Build and Run

    cargo install wasm-pack
    wasm-pack build --target web --out-dir www/pkg
    cargo install simple-http-server
    cd www
    simple-http-server --ip 127.0.0.1 -p 10333

Then point your browser at `localhost:10333`.
    
## Bugs
    
* Will create a second ball following the first in some
  circumstances.

## Acknowledgments

Thanks to all the various tutorials, guides, examples and
tool breakdowns — too many to list — that I borrowed from in
getting to this point.
