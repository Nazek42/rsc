# rsc
A dc-like command-like calculator written in Rust

## Usage

As a desk calculator:

    $ rsc -c '3.45 7.82 + .'
    11.27

As a pipe component:

    $ ...stuff... | rsc -c '$0 $2 * $1 + .' | ...other stuff...
    
## Language Reference

`rsc` implements a minimal stack-based language as its base.
Everything in a program is either a literal, a function call, a variable invocation, or an assignment.

### Literals

The only data type in `rsc` is a 64-bit floating point number. They are written exactly how you'd expect:

    1.2
    .07
    42
    -9
    -3.14
    -.105

All of the above are valid literals.

### Functions

Functions manipulate the stack. Below is a list of all functions currently available in `rsc`.

| Name | Description | Stack transform) |
|-|-|-|
|`+`|Adds two numbers|`a b -- c`|
|`-`|Subtracts two numbers|`a b -- c`|
|`*`|Multiplies two numbers|`a b -- c`|
|`/`|Divides two numbers|`a b -- c`|
|`^`|Raises one number to the power of another|`a b -- c`|
|`sqrt`|Takes the square root of a number|`a -- b`|
|`exp`|Raises Euler's number *e* to the power of a number|`a -- b`|
|`=`|Tests if two numbers are equal|`a b -- p`|
|`<`|Tests if a number is less than another|`a b -- p`|
|`>`|Tests if a number is greater than another|`a b -- p`|
|`<=`|Tests if a number is less than or equal to another|`a b -- p`|
|`>=`|Tests if a number is greater than or equal to another|`a b -- p`|
|`?`|Pushes the second value on the stack if the third value down is nonzero, otherwise the top value|`p t f -- t/f`|
|`.`|Prints a number to standard output|`a --`|

### Variables

Variables are referenced by prepending a `$` to their name, like so: `$var`.
When invoked, they place their value on the stack.

#### Assignment

You can create your own variables using the `:` operator.
`:` pops the top value off the stack and assigns it to the variable immediately following it.
For example, this is how you would set `x` to `3`:

    3 :x

You could then use `$x` later in the program.

#### Arguments

If not invoked in calculator (`-c`) mode,
`rsc` automatically reads a whitespace-separated set of numbers from standard input 
and lets you use them by putting a number after `$`, with the values starting at zero.
For example, this is how you would print the second value from stdin:

    $1 .

Note that you cannot assign to arguments; `:3`, while a cute smiley face, is a syntax error.

#### Pre-defined Variables

There are a few "constants" pre-defined.
I place "constants" in quotes because you can redefine them if you *really* want to.
They are enumerated below.

|Name|Mathematical name|Approximate value|
|-|-|-|
`E`|*e*|2.7183|
`PI`|*&pi;*|3.1415|
