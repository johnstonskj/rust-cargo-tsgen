# Package `cargo_tsgen`

Additional code generation tools for tree-sitter

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![Rust Workflow](https://github.com/johnstonskj/rust-cargo-tsgen/actions/workflows/rust.yml/badge.svg)](<https://github.com/johnstonskj/rust-cargo-tsgen/actions/workflows/rust.yml>)
[![Security Audit Workflow](https://github.com/johnstonskj/rust-cargo-tsgen/actions/workflows/security-audit.yml/badge.svg)](<https://github.com/johnstonskj/rust-cargo-tsgen/actions/workflows/security-audit.yml>)
[![Coverage Status](https://codecov.io/github/johnstonskj/rust-cargo-tsgen/graph/badge.svg?token=1HGN6M4KIT)](<https://codecov.io/github/johnstonskj/rust-cargo-tsgen>)
[![crates.io](https://img.shields.io/crates/v/cargo_tsgen.svg)](https://crates.io/crates/cargo_tsgen)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/cargo_tsgen)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-cargo-tsgen.svg)](<https://github.com/johnstonskj/rust-cargo-tsgen/stargazers>)

Add a longer description here.

## Example

TBD

```bash
 ❱ cargo tsgen --help
his command can generate additional, type-safe, features for tree-sitter bindings.

Usage: cargo-tsgen [OPTIONS] <COMMAND>

Commands:
  constants    Create a constants file from node-types.json
  wrapper      Create a type-safe wrapper around the tree-sitter CST using grammar.json
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Increase logging verbosity

  -q, --quiet...
          Decrease logging verbosity

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

- **constants**; this reads the tree-sitter generated file `src/node-types.json` and writes out a
  language-specific file containing constants for all node and field names.
- **wrapper**; this reads the tree-sitter generated file `src/grammar.json` and writes out a
  language-specific file containing `Node` wrappers for the grammar.
- **completions**; write out shell completions for the tool itself.

## Features

| Name      | Dependencies | Description                                                            |
|-----------|--------------|------------------------------------------------------------------------|
| *default* | std,         | Package default features.                                              |
| std       | alloc        | Enables the `std` library crate, the most common default.              |
| alloc     |              | Enables the `alloc` library crate, required in a `no_std` environment. |

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/rust-cargo-tsgen/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/rust-cargo-tsgen/blob/main/LICENSE-MIT).

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

For information on contributing to this project, see the following.

1. Project [Code of Conduct](https://github.com/johnstonskj/rust-cargo-tsgen/blob/main/CODE_OF_CONDUCT.md).
1. Project [Contribution Guidelines](https://github.com/johnstonskj/rust-cargo-tsgen/blob/main/CONTRIBUTING.md).
1. Project [TODO Items](<https://github.com/johnstonskj/rust-cargo-tsgen/issues>) in Issues.
1. Repository [Change Log](https://github.com/johnstonskj/rust-cargo-tsgen/blob/main/CHANGELOG.md).
