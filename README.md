# Ridiculously Useless Numeric Katastrophe - runk
> Please do not confuse this with Ronald's Universal Number Kruncher

* * *

## What is runk?
runk is a standard unix utility that does math on all computers on Earth!

## How come I never heard of runk?
A guy named Ronald made runk on a different timeline where runk is used the most.
I tought we were missing out so I decided to make it here as well.

[Evidence of runk existing](https://twitter.com/6thgrade4ever/status/1433519577892327424)

## Can I use runk to do math on all computers on Earth?
It's a work in progress, so it doesn't really work yet. Also as you might have 
guessed from the name I am not as confident in my programming skills as Ronald, 
so it might not be as great.

# Building
It's Rust project, just run `cargo build` or something.

# Usage
Give it a path to a `.runk` source file and runk will be happy to run it.
You can also shove the `.runk` file down the stadnard input 
and it should also work.

# Syntax
You can see these [examples](examples/).

Here is how a runk source file could look like when it's fully developed (Although everything will prablbly change during development):

```runk
#!/bin/env runk

# Simple expression
2 # Because expression is not assigned, it falls through as is displayed as debug output on stderr.

# Direct assignment
Z var0: 2 # var0 is equal to 2

# A jump mark
!START

# Assignment with Single-op expression
Z var1: (+ 1 $var0 3) # var1: 6

# Nested expression on previously declared variable
var1: (- $var0 (* $var1 10))


<file /path/to/file alias>
Z input: <in Z alias liberal> # reads word from file (no file means stdin)

# Multi-op expression
Z var2: [10 + $($var1 * 9) / $input] # var2: 64

<if (>= $var2 0)
    # Command
    <out "The result is $var2!">
><else
    var1: (+ $var1 1)
    <jump START>
>
```

*for whatever reason GitHub doesn't display proper syntax highlighting for runk I apologise.*
