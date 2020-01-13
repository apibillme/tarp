## Tarp - Code Coverage Tool

### Purpose

Provide a Rust Code Coverage tool

## Features

* Line coverage
* Uploading coverage to https://coveralls.io or https://codecov.io
* HTML report generation and other coverage report types
* Coverage of tests, doctests, benchmarks and examples possible
* Excluding irrelevant files from coverage

## Use

### Installation
``` sudo snap install tarp ```

### Ignoring code in files

Tarpaulin now allows you to ignore modules or functions using config attributes.
Below is an example of ignoring the main function in a project:

```Rust
#[cfg_attr(tarpaulin, skip)]
fn main() {
    println!("I won't be included in results");
}
```
