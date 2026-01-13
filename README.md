# eztest
A small tool for testing an executable against multiple test case arguments at once.

## Installation
If the current release does not have a binary that supports your platform you'll just have to build it yourself.
If you don't know how to do this, you will need to install [rustup](https://rustup.rs/) if you do not already have **cargo**.

Once that is installed and working you'll need to build it.
```
cargo build --release
```
The executable is automatically built at **./target/release** (. being the eztest folder).

## Usage
eztest [path to executable] [path to test case file]

- [path to executable] is the path to the app you want to test.
- [path to test case file] is the path to the file containing test arguments.

## Test Case File
The file is parsed as if each line is a new set of arguments to test so it has to be written as such.
Here is an example:
```
1920x1080
1920x
x1080
x
1920
aaa
19a20x1080
```
## 
It isn't the most user friendly but its very fast. Fun to make.
