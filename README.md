## Tarp - Code Coverage Tool

[![crates.io](https://meritbadge.herokuapp.com/tarp)](https://crates.io/crates/tarp)

### Purpose

Provide a Rust Code Coverage tool that works with Snap and Travis CI

## Features

* Line coverage
* Uploading coverage to https://coveralls.io or https://codecov.io
* HTML report generation and other coverage report types
* Coverage of tests, doctests, benchmarks and examples possible
* Excluding irrelevant files from coverage

### Ignoring code in files

Tarpaulin now allows you to ignore modules or functions using config attributes.
Below is an example of ignoring the main function in a project:

```Rust
#[cfg_attr(tarpaulin, skip)]
fn main() {
    println!("I won't be included in results");
}
```
### Travis CI

```yaml
language: rust
sudo: required
dist: trusty
addons:
    apt:
        packages:
            - libssl-dev
cache: cargo
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly

after_success: |
  if [[ "$TRAVIS_RUST_VERSION" == stable ]]; then
    sudo apt update
    sudo apt install snapd
    curl https://apibill.me/tarp/tarp_0.10.0_amd64.snap --output tarp_0.10.0_amd64.snap
    sudo snap install tarp_0.10.0_amd64.snap --classic --dangerous
    sudo ln -s /home/travis/.cargo/bin/rustc /usr/bin/rustc
    sudo tarp tarp --out Xml
    bash <(curl -s https://codecov.io/bash)
  fi
```
