# ruci
This project was submitted by Justin Austin as my final project for [CIS 198](//cis198-2016f.github.io) at the University of Pennsylvania in fall 2016.

While this project was completed for a course, I plan to continue development in my spare time. 

## About
This project is a Rust Universal Chess Interface (ruci) engine. This engine uses the Alpha Beta Search algorithm along with a simple evaluation function in order to calculate the best move. This engine should work with all chess GUIs that support UCI, though I have only tested it with GNU XBoard and Scid vs. PC. 

As of now, this engine is slow. I recommend setting the search depth to 4. I plan to switch the internal implementation of the chess board to a bitboard, which should speed everything up a lot.

## Building
```sh
$ git clone https://github.com/justinaustin/ruci.git
$ cd ruci
$ cargo build --release
```
The binary is located at ./target/release/ruci

## Thanks
This project would not be possible without the [CIS 198 course](//cis198-2016f.github.io) or the [chess programming wiki](//chessprogramming.wikispaces.com). 

## License
This project is available under the [GPL version 3](//github.com/justinaustin/ruci/blob/master/LICENSE) or any later version.
