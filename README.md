# RsPython

Rust implementation of the python language.

ATTENTION: project in idea phase!

[![Build Status](https://travis-ci.org/windelbouwman/rupy.svg?branch=master)](https://travis-ci.org/windelbouwman/rupy)

## Introduction

This package aims to implement the python 3 language in rust. The other
current implementations of the python3 language are:

- CPython (reference implementation in C)
- PyPy (python implemented in python)
- IronPython3 (python implementation in .NET)
- MicroPython (python implementation in C for microcontrollers)

Possible use cases for RsPython:

- Compile RsPython to webassembly and run python3 scripts in the browser
- Port python to Redox-os.

## Usage:

cargo run demos/demo.py

