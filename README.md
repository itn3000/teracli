# Command line tool for Tera template engine

This is the command line frontend for [Tera template engine](https://tera.netlify.app/docs)

# Get binary

## From github

you can get binary from [Release page](https://github.com/itn3000/teracli/releases).
There is no prerequisits.

## From cargo

You must install [Cargo](https://doc.rust-lang.org/cargo/), which usually install from [rustup](https://rustup.rs/).
And then you run following command.
`cargo install --git https://github.com/itn3000/teracli.git --tag [version tag]`

# Run

run binary with three options

|name|description|
|----|-----------|
|-i,--input|input file(or STDIN by default)|
|-o,--output|output file(or STDOUT by default)|
|-j,--json-parameter|parameter json file|
|-k,--key-parameter|keyvalue parameter pair delimited by '=', overwrite json parameter if same key|

## Examples

* read from "x.template", replace "abc" to "def", "ghi" to "jkl", output to STDOUT
    * `teracli -i "x.template" -k abc=def -k ghi=jkl`
* read from "x.template", read parameters from "params.json", output to "x-replaced.txt"
    * `teracli -i "x.template" -j "params.json" -o "x-replaced.txt"`