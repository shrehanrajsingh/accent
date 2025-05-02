# Accent.js 🚀

> A lightweight, blazing fast JavaScript interpreter for Project J, written entirely in Rust

Accent.js is an experimental JavaScript interpreter that aims to provide a simple, efficient runtime for JavaScript with native performance. Built from the ground up in Rust, it focuses on speed, memory safety, and ease of embedding.

## 📋 Table of Contents

- [Overview](#-overview)
- [Project Structure](#-project-structure)
- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Examples](#-examples)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgements](#-acknowledgements)


## 🔍 Overview

Accent.js aims to be a compliant JavaScript interpreter with a focus on performance and embedding capabilities. Currently in early development, it already supports basic language features with more to come. The interpreter is written entirely in Rust, taking advantage of Rust's performance and safety guarantees.

## 📁 Project Structure

The project is divided into two main components:

```
accent/
├── accent/        # Core JavaScript interpreter library
└── accent-exec/   # Executable binary that runs JavaScript files
```

| Component | Description |
| :-- | :-- |
| `accent/` | Core library that implements the JavaScript language specification |
| `accent-exec/` | Command-line tool that takes a JavaScript file and executes it |

## ✨ Features

> **Note:** Accent.js is currently in early stages of development. The feature set is expanding rapidly.

### Current Features

- ✅ **Variable Declaration** - Support for `let` variable declarations
- ✅ **Basic Data Types**:
    - Integers (`123`)
    - Floating-point numbers (`3.14`)
    - Strings (`"Hello, world!"`)
    - Booleans (`true`, `false`)
    - `undefined` value
- ✅ **Arithmetic Operations**:
    - Addition (`+`)
    - Subtraction (`-`)
    - Multiplication (`*`)
    - Division (`/`)
- ✅ **Functions**:
    - Function declarations (`function name(args) {...}`)
    - Function calls with arguments
    - Recursive function support
- ✅ **Built-in Functions**:
    - `print()` - Output values to the console


### Current Limitations

- ⚠️ Every statement **must** end with a semicolon
- ⚠️ Only classic function declarations are supported (no arrow functions yet)
- ⚠️ Limited standard library functionality


## 📥 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Cargo (comes with Rust)


### Building from Source

```bash
# Clone the repository
git clone https://github.com/shrehanrajsingh/accent.git
cd accent

# Build the project
cargo build --release

# The binary will be located at
# ./target/release/accent-exec
```


## 🚀 Usage

After building the project, you can run JavaScript files using the `accent-exec` binary:

```bash
# Run a JavaScript file
./target/release/accent-exec path/to/your/script.js

# Or if you've installed it to your path
accent-exec path/to/your/script.js
```


## 📝 Examples

### Basic Arithmetic

```javascript
let a = 10;
let b = 20;
let c = a * b;
print(a, b, c);
```

**Output:**

```
10 20 200
```


### Recursive Function

```javascript
function recursive(a) {
    print(a);
    recursive(a + 1);
}

recursive(1);
```

**Output:**

```
1
2
3
...
3777
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

> **Note:** The stack overflow is expected behavior as there's no base case in the recursive function.

### Function with Return Value

```javascript
function add(a, b) {
    return a + b;
}

let result = add(5, 7);
print("The sum is:", result);
```

**Output:**

```
The sum is: 12
```


## 🗺️ Roadmap

The following features are planned for future releases:


| Feature | Status | Priority |
| :-- | :-- | :-- |
| **Classes and Objects** | Planned | High |
| **Native JSON Parser** | Planned | Medium |
| **String Methods** | Planned | High |
| **Web Server Support** | Planned | Medium |
| **Array Methods** | Planned | High |
| **Error Handling** | Planned | High |
| **ES6+ Features** | Planned | Medium |
| **Package Manager Integration** | Planned | Low |

## 🤝 Contributing

Contributions are welcome and appreciated! Here's how you can contribute:

1. **Fork** the repository
2. **Create** a new branch:

```bash
git checkout -b feature/amazing-feature
```

3. **Make** your changes
4. **Commit** your changes:

```bash
git commit -m 'Add some amazing feature'
```

5. **Push** to the branch:

```bash
git push origin feature/amazing-feature
```

6. **Submit** a pull request

Please ensure your code follows the project's coding standards and includes appropriate tests.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- Created with ❤️ by [shrehanrajsingh](https://github.com/shrehanrajsingh)
- Inspired by modern JavaScript runtimes
- Built with Rust's incredible ecosystem

---

<div>
  <img>
  <br>
  <sub>© 2025 Accent.js Project. All rights reserved.</sub>
</div>

[^1]: https://github.com/othneildrew/Best-README-Template

[^2]: https://www.hatica.io/blog/best-practices-for-github-readme/

[^3]: https://github.com/matiassingers/awesome-readme

[^4]: https://shields.io/badges/crates-io-version

[^5]: https://shields.io/badges/docs-rs

[^6]: https://gist.github.com/kofiav/c1059e1075b67582e86b07aa9759e20d

[^7]: https://github.com/zaszi/rust-template/blob/master/README.md

[^8]: https://github.com/serde-ml/serde/blob/main/README.md

[^9]: https://www.makeareadme.com

[^10]: https://shields.io/badges/crates-io-downloads-latest-version

[^11]: https://github.com/rust-lang/rust-by-example/blob/master/README.md

[^12]: https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc

[^13]: https://shields.io/badges/crates-io-msrv

[^14]: https://rustprojectprimer.com/documentation/repository.html

[^15]: https://www.drupal.org/docs/develop/managing-a-drupalorg-theme-module-or-distribution-project/documenting-your-project/readmemd-template

[^16]: https://shields.io/badges/crates-io-size

[^17]: https://www.reddit.com/r/opensource/comments/txl9zq/next_level_readme/

[^18]: https://docs.rs/rsbadges

[^19]: https://coding-boot-camp.github.io/full-stack/github/professional-readme-guide/

[^20]: https://shields.io

[^21]: https://github.com/jehna/readme-best-practices

[^22]: https://dev.to/documatic/awesome-readme-examples-for-writing-better-readmes-3eh3

[^23]: https://readme.so

[^24]: https://tilburgsciencehub.com/topics/collaborate-share/share-your-work/content-creation/readme-best-practices/

[^25]: https://www.youtube.com/watch?v=Nj87GEXxhjc

[^26]: https://www.readme-templates.com

[^27]: https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes

[^28]: https://www.reddit.com/r/github/comments/uulygm/what_are_some_really_nice_github_profile_readmes/

[^29]: https://www.youtube.com/watch?v=G-EGDH50hGE

[^30]: https://www.freecodecamp.org/news/how-to-write-a-good-readme-file/

[^31]: https://project-awesome.org/matiassingers/awesome-readme

[^32]: https://docs.github.com/github/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax

[^33]: https://docs.rs/about/badges

[^34]: https://gist.github.com/lukas-h/2a5d00690736b4c3a7ba

[^35]: https://github.com/heyvito/shield-maker

[^36]: https://github.com/badges/shields/blob/master/doc/TUTORIAL.md

[^37]: https://stackoverflow.com/questions/65819996/shield-io-license-badges-and-github-license-badge-not-working

[^38]: https://crates.io/crates/badge-maker

[^39]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge\&labelColor=555555\&logoColor=white\&logo=data%3Aimage%2Fsvg+xml%3Bbase64%2CPHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

[^40]: https://shields.io/badges/git-hub-license

[^41]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge\&labelColor=555555\&logo=rust

[^42]: https://img.shields.io/badge/docs.rs-gpu--alloc-66c2a5?style=for-the-badge\&labelColor=555555\&logoColor=white

[^43]: https://shields.io/badges/npm-license

[^44]: https://crates.io/crates/shields-io

[^45]: https://github.com/serde-rs/serde/blob/master/README.md

[^46]: https://docs.rs/tokio

[^47]: https://blog.guillaume-gomez.fr/articles/2019-04-13+Keeping+Rust+projects'+README.md+code+examples+up-to-date

[^48]: https://serde.rs

[^49]: https://git.picodata.io/core/tarantool-module/-/blob/e3c8a784f23de10153346fc6b665f4d666477b06/examples/tokio-hyper/README.md

[^50]: https://docs.rs/pretty-readme

[^51]: https://chromium.googlesource.com/external/github.com/bolinfest/serde-jsonrc/+show/refs/heads/upstream/supports-comments-option/README.md

[^52]: https://github.com/tokio-rs/tokio/blob/master/README.md

[^53]: https://rust-lang.github.io/rust-project-goals/admin/samples/main-readme.html

[^54]: https://chromium.googlesource.com/external/github.com/serde-rs/json/+/6434761d775be4b287381599e283478510b3c765

[^55]: https://tokio.rs/tokio/tutorial

[^56]: https://crates.io/crates/generate-readme

[^57]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

[^58]: https://www.reddit.com/r/rust/comments/bck35j/keeping_rust_projects_readmemd_code_examples/

[^59]: https://github.com/tokio-rs/tokio/blob/master/examples/README.md

[^60]: https://doc.rust-lang.org/rust-by-example/

