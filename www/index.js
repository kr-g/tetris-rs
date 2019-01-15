import * as wasm from "tetris-rs";

function $id( id ){
    return document.getElementById(id);
}

function leave( id ){
    $id( id ).blur();
}

$id("reset").onclick= function(){ reset(); leave("reset"); } ;

$id("stopplay").onclick= function(){ stopplay_pressed(); leave("stopplay"); } ;

$id("left").onclick= function(){ inputHandler ( { keyCode: 37 } ); } ;
$id("turn").onclick= function(){ inputHandler ( { keyCode: 32 } ); } ;
$id("right").onclick= function(){ inputHandler ( { keyCode: 39 } ); } ;
$id("down").onclick= function(){ inputHandler ( { keyCode: 40 } ); } ;

window.addEventListener('keydown',inputHandler,false);

function inputHandler(e) {
    //console.log( e.keyCode );
    if( gameover )
        return;
    if( paused && e.keyCode != 80 ){
        // continue game
        stopplay_pressed();
        return;
    }
    switch( e.keyCode ){
        case 32: 
        case 38: 
            game.turn();
            break;
        case 37: 
            game.left();
            break;
        /*case 38: 
            game.draw();
            break;*/
        case 39: 
            game.right();
            break;
        case 40: 
            if( paused ) return;
            nextTick();
            break;
        case 80: 
            stopplay_pressed();
            break;
        default:
            break;
    }
}

var game;
var gameover;
var paused;

function nextTick(){
    if( gameover )
        return;
    gameover = game.next_tick( Math.random() );
    game.draw_next_stone();
    if( gameover ){
        paused = true;
    }
}

function reset(){
    game = wasm.TheGame.new( Math.random(), Math.random() );
    game.draw();
    game.draw_next_stone();
    gameover = false;
    paused = true;
    clearTimeout(timeclk);
}

reset();

var timeclk;

function stopplay_pressed(){
    clearTimeout(timeclk);
    if( paused ){
        autoplay();
        $id("pause-label").style = "display: none;";
    } else {
        $id("pause-label").style = "display: block;";
    }
    paused = !paused;
}

function autoplay(){
    clearTimeout(timeclk);
    nextTick();
    timeclk = setTimeout( autoplay, 1000 );
}


