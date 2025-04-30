# 🐍 Nadra - Modern Scripting Language 🐍

**DISCLAIMER: Nadra is currently in early development and thus could contain bugs.**

## What is Nadra?
Nadra is intended to be an alternative syntax to Python 3, to which it is transpiled. The name *Nadra* comes from the Old Norse word 'naðra' */'naðra/* which means 'snake'. Nadra is for anyone who often uses Python but is not a fan of indentation being used to mark code blocks, or is just not fond of the Python syntax.

## What Nadra is Not
Nadra is not intented to be a replacement for Python. Python is a very useful language used in a variety of fields, while Nadra is just an alternative syntax for Python, which in turn gets transpiled into Python.

## Hello, World!
Writing a 'Hello, World!' program in Nadra is pretty simple. In fact, the most basic way of implementing 'Hello, World!' in Nadra is identical to Python.

```nadra
print("Hello, World!")
```

Another way to implement it which is similar to Python but not identical is:

```nadra
if __name__ == "__main__" then
    print("Hello, World!")
endif
```

## Installation
To install Nadra, you will need a few things on your system.
- Cargo - used to build Nadra
- Python 3 - Used to run the outputted Python code created by the transpiler.

If you have these installed, then the next step to install Nadra is obtaining a copy of the source code. To do this, run:

```sh
git clone https://github.com/brayner05/nadra.git
```

Once you have cloned this repository, you can build Nadra:

```sh
cd nadra
cargo build --release
```

From here, you can begin writing Nadra code.


## The Transpiler
Nadra does not currently run on it's own, but rather any Nadra code gets transpiled (converted) to Python code, which can then be run via whichever Python environment you wish.

### How to invoke the transpiler
Imagine you have a Nadra project layed out as follows:

```
.
├── README.md
├── .gitignore
├── main.ndr
```

To compile `main.ndr` to Python, invoke the transpiler:

```sh
nadrac main.ndr main.py
```

And of course, to run the file,

```3
python3 main.py
```

It is often helpful to simplify this build system. To do so, a variety of tools can be used. *GNU Make* is one such tool.


## The Basics
Now that Nadra is installed, and we know how to compile our code, let us go over some of the basic features of Nadra.

### Functions
Creating a function in Nadra is quite similar to Python, but with a more familiar syntax for those who have used languages such as *Ruby*, *Bash*, *Fish*, etc.

Nadra functions are layed out as so:
```
def <FunctionName> (<ParameterName>*)
    <body>
enddef
```

For example,

```nadra
def greet(name) -> None
    print("Hello, " + name + " !")
enddef
```

### Control Flow
Control flow in Nadra is similar to most languages.

```nadra
if <condition> then
    <body>
else
    <body>
endif
```

For example,

```nadra
if n % 2 == 0 then
    print("Even")
else
    print("Odd")
endif
```

**As of now, Nadra does not support looping. This is temporary, as loops are one of the main features in development. Nadra will feature two main types of loops:**

#### For In
The first kind of loop supported by Nadra is the 'for in' loop.

```nadra
for <item> in <iterable> do
    <body>
done
```

For example, 

```nadra
def factorial(n) -> int
    result = 1
    for i in 1..(n + 1) do
        result *= i
    done
    return result
enddef
```

#### While

### Custom Types
One of the major features of Nadra is a more satisfying way of creating custom types. Nadra supports `struct` and `enum` as a way of creating your own types.

```nadra
struct Vector
    x: int
    y: int

    def __repr__(self) -> String
        return "[" + self.x + "," + self.y + "]"
    enddef

    def dot(self, other) -> String
        return self.x * other.x + self.y * other.y
    enddef
endstruct


enum RenderModes
    Detailed,
    Minimal,
    Performance
endenum
```

### Lambda Expressions
One of the upcoming features of Nadra is it's lambda expressions. Nadra's lambda expressions take a familiar form for anyone who has used *Java*, *C#*, or even *TypeScript*.

```nadra
# Creates a list containing the square # of numbers in `numbers`.

numbers.map((n) -> n ** 2)
```

## Contribution
It is my intention to make Nadra free, open-source, and accept contributions to the project.

As of now, Nadra is still in the early stages of development, so while contributions are welcome, they are not the main priority of the project as of now. 

Once Nadra is ready for release, contributions will remain welcome and appreciated.


## Nadra Wishlist
The following are some features that are being considered for Nadra, some of which are already in development.

| Feature | State | Description |
| --------|-------|-------------|
|   Loops      |   **Development**    |  Support for while loops and for-in loops.          |
|Ranges|**Development**|Create the equivalent of Python's `range(n)` with the Nadra `0..n`|
|Lambda Expressions|**Development**|Anonymous functions that are useful for functional programming and more.
|Strong(er) Typing|**Planned**|Allowing for strongly-typed variables and a type-checking system inside the Nadra compiler.
|Nadra VM|**Consideration**|An implementation of Nadra that directly runs the Nadra code without converting to Python.
|Performance Mode|**Consideration**|Leverage the *Mojo* language for a *Python*-style syntax while being compiled to machine code.
