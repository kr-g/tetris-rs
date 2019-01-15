# tetris-rs 

a draft implemenation of tetris unsing rust, wasm and html canvas.
most of the code is just a hack...

# how it works

the main rust code is under the src folder. the ww folder contains the html and javascript (and pluming code required for wasm-bindgen). 

# how to deploy 

i used the output of `npx webpack` (as described https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html) and moved the generated code from www/dist to the docs folder so that it's available via github pages

# play in your browser

just open https://kr-g.github.io/tetris-rs/

it's possible to play tetris in the browser on github pages, just open https://kr-g.github.io/tetris-rs/ 
use the keyboard or the buttons to control the blocks (might not work proper on mobile browsers due to screen size... sorry for that...)

# rust and wasm reading

https://rustwasm.github.io/book/

https://rustwasm.github.io/wasm-bindgen/introduction.html

