<img src="logo.gif">

[![Rust](https://github.com/OndrejZobal/runk/actions/workflows/rust.yml/badge.svg)](https://github.com/OndrejZobal/runk/actions/workflows/rust.yml)

# Ridiculously Useless Numeric Katastrophe

> Please do not confuse this with Ronald's Universal Number Kruncher

## What is runk?
runk is a standard Unix utility that does math on all computers on Earth!

## How come I never heard of runk?
A guy named Ronald made runk on a different timeline where runk is used the most.
I thought we were missing out so I decided to make it here as well.

[Evidence of runk existing](https://twitter.com/6thgrade4ever/status/1433519577892327424)

## Can I use runk to do math on all computers on Earth?
It's a work in progress, so it doesn't really work yet. Also as you might have
guessed from the name I am not as confident in my programming skills as Ronald,
so it might not be as great.

# Building
It's Rust project, just run `cargo build` or something.

# Usage
Give it a path to a `.runk` source file and runk will be happy to run it.
You can also shove the `.runk` file down the standard input
and it should also work.

# What works
The interpreter is not yet fully implemented, but some things already work. You can have a look at my [test file](examples/test.runk) everything in this file should always work and the interpreter should exit with 0.

My latest breakthrough is *tricking* runk into displaying the Fibonacci sequence!

``` runk
#!/bin/env runk

# =========
# Fibonacci
# =========
Z max: 1000
Z n1: 1
Z n2: 0
Z helper: 0
N order: 1

!loop                                               # A lable to return to at the beginning of every iteration.
(out $order "th Fibonacci number is" $n2 "!")       # Prints previous number.
helper: $n1                                         # Saving current number to herlper
n1: (+ $n1 $n2)                                     # Sets current number to: current number + previous number
n2: $helper                                         # Copyies helper into previous number
order: (+ $order 1)                                 # Increment order.
(jumpnz (< $n2 $max) $lable)                        # Jumps if condition is not equal to zero. 

(out "And that's it!")
```

# Syntax
You can see these [examples](examples/).

Here is how a runk source file could look like when it's fully developed (Although everything will probably change during development):

```runk
#!/bin/env runk

# Simple expression
2 # Because expression is not assigned, it falls through and is displayed as debug output on stderr.

# Direct assignment
Z var0: 2 # var0 is equal to 2

# Assignment with Single-op expression
Z var1: (+ 1 $var0 3) # var1: 6

# Nested expression on previously declared variable
var1: (- $var0 (* $var1 10))


<file /path/to/file alias>
Z input: <in Z alias liberal> # reads word from file (no file means stdin)

# Multi-op expression
Z var2: [10 + $($var1 * 9) / $input] # var2: 64

# A lable
!start

<if (>= $var2 0)
    # Command
    <out "The result is $var2!">
><else
    var1: (+ $var1 1)
    (jump START)
>
```

*for whatever reason GitHub doesn't display proper syntax highlighting for runk I apologise.*

* * *

*Logo by <https://cooltext.com/>*
