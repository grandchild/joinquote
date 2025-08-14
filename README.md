# joinquote

A CLI tool to join consecutive lines of input, optionally with configurable
quoting. 


## Usage

```
Usage: joinquote [<separator>] [-q <quote>] [-Q]

join lines from stdin with configurable quoting

Positional Arguments:
  separator         separator (default: ", ")

Options:
  -q, --quote       quote string, if "(", "[", "{", or "<" will be auto-paired
  -Q, --no-quotes   don't quote lines
  --help, help      display usage information
```


### Matched Parentheses

The tool automatically handles matched parentheses pairs.
You can specify any of these formats:
- `(`, `)`, or `()` → results in `(item)`
- `[`, `]`, or `[]` → results in `[item]`
- `{`, `}`, or `{}` → results in `{item}`
- `<`, `>`, or `<>` → results in `<item>`


### Examples

```bash
# Basic usage - join lines with double quotes and comma-space separator
$ seq 5 | joinquote
"1", "2", "3", "4", "5"

# Custom separator
$ seq 5 | joinquote " | "
"1" | "2" | "3" | "4" | "5"

# Disable quoting
$ seq 5 | joinquote -Q
1, 2, 3, 4, 5

# Custom quote character
$ seq 5 | joinquote -q "'"
'1', '2', '3', '4', '5'

# Parentheses and brackets (matched pairs)
$ seq 5 | joinquote -q "("
(1), (2), (3), (4), (5)

$ seq 5 | joinquote -q "]"
[1], [2], [3], [4], [5]

$ seq 5 | joinquote " | " -q "{}"
{1} | {2} | {3} | {4} | {5}

# Combined options
$ seq 5 | joinquote " -> " -Q
1 -> 2 -> 3 -> 4 -> 5
```


## License

CC0 - free software.
To the extent possible under law, all copyright and related or neighboring
rights to this work are waived. See the LICENSE file for more information.
