# clopy

it's like git shallow clone, but it only makes a copy. inspired by degit.

great for ci, templating, compiling from source without full git history, etc.

## Getting Started

### Installation

`cargo install clopy`

### Use

```{shell}
clopy [host/]owner/repo[:ref] [destination]

# examples:
clopy kjpark/clopy
clopy github.com/kjpark/clopy:dev
```

required arguments:
- `owner` - repo owner
- `repo` - repo name

optional arguments:
- `host/` - `github.com` (default) or `gitlab.com`
- `:ref` - branch, tag, or commit
- `destination` - output path for extracted folder

## Help

```{shell}
> clopy --help
...
USAGE:
    clopy [OPTIONS] <SOURCE> [DESTINATION]

ARGS:
    <SOURCE>         "[host/]owner/repo[:branch|:tag|:commit]"
    <DESTINATION>    "output/path"

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    verbose output?
    -V, --version    Print version information
```

## Authors

Jedidiah Park [(kjpark)](https://github.com/kjpark)

## Version History

- 0.1.2
  - refactor to use modules
  - use regex in parsing logic
- 0.1.1
  - first functional release
- 0.1
  - init cargo test release

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Inspired by the original [degit.](https://github.com/Rich-Harris/degit)
