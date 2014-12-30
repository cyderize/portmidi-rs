portmidi-rs [![Build Status](https://travis-ci.org/cyderize/portmidi-rs.svg)](https://travis-ci.org/cyderize/portmidi-rs)
===========

PortMidi bindings for Rust

This is a work-in-progress mainly for my own experimental purposes. [rust-portmidi](https://github.com/musitdev/rust-portmidi) provides a better tested set of bindings.

## Usage

To add the library's Git repository to a Cargo project, add this to your Cargo.toml:

```INI
[dependencies.portmidi]

git = "https://github.com/cyderize/portmidi-rs.git"
```

And add ```extern crate portmidi;``` to your project.

## License

### The MIT License (MIT)

Copyright (c) 2014 Cyderize

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
