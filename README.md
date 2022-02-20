# wordle
Author: Nathan McIntosh

### About
This rust program helps solve the Wordle game

### Compiling and Running
1. Install the [Rust programing language](https://www.rust-lang.org)
1. Clone this repo to a location on your computer: `git clone https://github.com/natemcintosh/wordle.git`
1. Build in release mode: `cargo build --release`
1. Run the binary and ask for help: `./target/release/wordle -h`
1. To run the efficiency tests and see the scores for each of the available solvers, run
with `--run-tests`
1. To have it help you, run with no arguments. Enter what the wordle site has told you.
If for example, you typed `HELLO` into wordle, and it told you that their colors were black, yellow, black, black, green, in that order
you would then type in your terminal running the program `hb ey lb lb og`. The pairs are the letter
and it color: `b` for black, `y` for yellow, and `g` for green.
1. The program will then tell you what words are available and ask you to enter the new data you got
from the website
1. Repeat the process until you win

### Future Improvment
Use information theory to be more efficient. Using 3Blue1Brown's
[video](https://www.youtube.com/watch?v=v68zYyaEmEA).
- For each word, get all the possible patterns we could see; there are 3 for each letter
space, and five letters, so we have $3^5=243$ total patterns. For each of those patterns,
there are some number of words that match the pattern; sometimes many, sometimes very few.
The "most likely" outcomes give you the least new information, and vice versa.
- $E[info]=\Sigma_x \left(p(x) \cdot something\right)$.
- Say the probably of some event $x$ is $\frac{3}{2000}$, i.e. this pattern gives only
gives three possible words that match the pattern of the original 2000. We can say that
$p(pattern) = \frac{3}{2000}$. The information, $I$, in that pattern is $$I=log_2(1/p)=-log_2(p)$$
So the information in our case is $-log_2(3/2000)=9.380821783940931$
- Now we know how to calculate the $something$ in the calculation for expected
information, we can calculate $E[info]$. 
- As we're playing the game, the word population will dwindle, so we'll have to 
recalculate this for each word, each time. 
