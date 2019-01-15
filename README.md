# tetris-rs 

a draft implemenation of tetris using rust, wasm and a html canvas.
most of the code is just a hack... enjoy ;-)

# how it works

the main rust code is under the src folder. the www folder contains the html and javascript (and plumbing code required for wasm-bindgen). 

# how to deploy 

i used the output of `npx webpack` (as described https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html) and moved the generated code from www/dist to the docs folder so that it's also available via github pages

# play in your browser

just open https://kr-g.github.io/tetris-rs/

it's possible to play tetris in the browser on github pages, just open https://kr-g.github.io/tetris-rs/ 
use the keyboard or the html buttons on the page to control the blocks (might not work proper on mobile browsers due to screen size... sorry for that... but i only wanted to try wasm and dont wanted to build a full fledged game running on all devices... )

# rust and wasm reading

https://rustwasm.github.io/book/

https://rustwasm.github.io/wasm-bindgen/introduction.html

