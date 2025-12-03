<div align="center">

  # Ascii Binary Compiler
Transform your `.bapple` media into actual executable files.


</div>

## Usage
```sh
# To use this, you're going to need a `.bapple` file. You can get one of those by
# converting an MP4 using `asciic` (https://github.com/S0raWasTaken/bad_apple/tree/master/asciic)
asciic -c video.mp4
# Outputs: ./video.bapple
abc video.bapple
# Outputs ./video
```

## Installation
I'll not provide static binaries for this project, because you'll need `cargo` and `rust` to use it anyway.
```sh
cargo install --git https://github.com/S0raWasTaken/ascii-binary-compiler abc
```

## License
MIT
