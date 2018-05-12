# RsPython

Rust implementation of the python language.

ATTENTION: project in idea phase!

[![Build Status](https://travis-ci.org/windelbouwman/rspython.svg?branch=master)](https://travis-ci.org/windelbouwman/rspython)

## Introduction

This package aims to implement the python 3 language in rust.
Current implementations of the python3 language are:

- CPython3 (reference implementation in C)
- PyPy3 (python implemented in python)
- IronPython3 (python implementation in .NET)
- MicroPython (python implementation in C for microcontrollers)

Use cases for RsPython:

- Compile RsPython to webassembly and run python3 scripts in the browser
- Port python to Redox-os.
- Provide an alternative implementation next to CPython
- Combine rust with python in a more natural way

## Usage:

    $ git clone https://github.com/windelbouwman/rspython
    $ cd rspython
    $ cat demos/simple.py
    print('a=', 2 - 22)
    $ cargo run demos/simple.py
    a= -20
    $ python demos/simple.py
    a= -20

To get a whole lot of logging, use:

    $ RUST_LOG=trace ./target/debug/rspython demos/simple.py
 
## Planning:

- Phase 1: Minimal demo with built in print function and integer variables
- Phase 2: Add function definitions
- Phase 3: Add class definitions
