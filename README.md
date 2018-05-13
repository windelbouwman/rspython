# RsPython

Rust implementation of the python language :snake: :scream: :metal:.

ATTENTION: project in prototyping phase!

[![Build Status](https://travis-ci.org/windelbouwman/rspython.svg?branch=master)](https://travis-ci.org/windelbouwman/rspython)
[![Join the chat at https://gitter.im/ppci-dev/rspython](https://badges.gitter.im/ppci-chat/rspython.svg)](https://gitter.im/ppci-chat/rspython)

## Introduction

This package aims to implement the python 3 language in rust.
Current implementations of the python3 language are:

- CPython3 (reference implementation in C)
- PyPy3 (python implemented in python)
- IronPython3 (python implementation in .NET)
- MicroPython (python implementation in C for microcontrollers)
- RsPython (you are reading the readme now)

Use cases for RsPython:

- Compile RsPython to webassembly and run python3 scripts in the browser
- Port python to Redox-os.
- Provide an alternative implementation next to CPython
- Combine rust with python in a more natural way

## Usage

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
 
## Planning

- Phase 1: Minimal demo with built in print function and integer variables
- Phase 2: Add function definitions
- Phase 3: Add class definitions

## Design

The design follows that of CPython. The code is first parsed into an AST (abstract syntax tree).
Then it is compiled into bytecode. This bytecode is then executed by a virtual machine. The
virtual machine has access to several built-in python types such as `int`, `tuple`, `list`,
`set`, `dict` and `iter`.

### Parsing

A handwritten lexer is combined with a [LALRPOP](https://github.com/lalrpop/lalrpop)
generated parser. To deal with indentation,
the lexer inserts `INDENT` and `DEDENT` tokens at the appropriate points.

### Compilation

The AST is transformed in bytecode. The available bytecode operations are loosely based upon
[the ones in CPython](https://docs.python.org/3/library/dis.html#python-bytecode-instructions)

## Ideas / notes

- Instead of compiling to bytecode, the abstract syntax tree can be compiled into rust code and
  compiled with the language runtime objects. This would give a speedup?
- A compatible bytecode would be beneficial in order to be able to exchange bytecode files
  between CPython / micropython.
