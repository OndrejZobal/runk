# List of runk functions:
- `+`: Sums unlimited numbers.
- `*`: Multiplies unlimited numbers.
- `-`: Subtracts unlimited number of numbers from the first number.
- `/`: Divides unlimited number of numbers from the first number.

- `<`: Returns 1 if first number is less then the second one or 0 otherwise.
- `>`: Returns 1 if second number is less then the first one or 0 otherwise.
- `=`: Returns 1 if first and second arguments are equal or 0 otherwise.
- `=!`: Returns 1 if first and second arguments are not equal or 0 if they are.
- `<=`: Returns 1 if first number is less or equal then the second one or 0 otherwise.
- `>=`: Returns 1 if second number is less or equal then the first one or 0 otherwise.

- `in`: Reads a line from the standard input. No arguments.

- `int`: Tries to convert the argument to `Int`.
- `nat`: Tries to convert the argument to `Nat`.

- `bin`: Converts a non-zero number to 1
- `not`: Converts a non-zero number to 0 and zero to 1.
- `and`: Logical and of all supplied numbers.
- `or`: Logical or of all supplied numbers.

- `go`: Unconditional jump to a label as first argument.
- `goif`: Jump if first argument is not 0 to a label in second argument.

- `err`: Print to standard error.
- `cat`: Concat args as text.
- `cats`: Concat lines as text with space padding.
- `line`: Concat lines as text and append new-line.
- `lines`: Concat lines as text with space padding and append new-line.

- `exit`: Exits the program with exit code specified in the first argument as a number.
