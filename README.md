# wordle
Author: Nathan McIntosh

### About
This rust program helps solve the Wordle game

### Compiling and Running
1. Make sure you have the [Rust programing language](https://www.rust-lang.org) installed
1. Clone this repo to a location on your computer: `git clone https://github.com/natemcintosh/wordle.git`
1. Build in release mode: `cargo build --release`
1. Run the binary: `./target/release/wordle`
1. Enter what the wordle site has told you. If for example, you typed `HELLO` into 
wordle, and it told you that their colors were black, yellow, black, black, green, in that order
you would then type in your terminal running the program `hb ey lb lb og`. The pairs are the letter
and it color: `b` for black, `y` for yellow, and `g` for green. 
1. The program will then tell you what words are available and ask you to enter the new data you got 
from the website
1. Repeat the process until you win
