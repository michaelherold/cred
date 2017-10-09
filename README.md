# cred - A simple credential store

[![Build Status](https://travis-ci.org/michaelherold/cred.svg?branch=add-ci)](https://travis-ci.org/michaelherold/cred)

**Warning: This is pre-alpha software and should not be used by anyone**

Have you ever wondering what password managers do? Have you ever wondered if you could build your own? Or maybe you’ve been frustrated with the security vulnerabilities of some of the major players in the space.

All three of these describe me. So I’m setting out to do just that: build my own password manager. With the [Keybase file system], we should be able to make a shared credential store that is store in an encrypted fashion without much trouble. (He says, knowing this is pure naiveté.)

[Keybase file system]: https://keybase.io/docs/kbfs

## Installation

There are no distributed builds of Cred yet. To build it yourself, first clone the Git repository:

    git clone https://github.com/michaelherold/cred.git

You can now use the setup task to get your build environment in tip-top shape:

    bin/setup

Once you have Rust, you will be able to build Cred with the following command:

    cargo build

This will build Cred with debugging symbols. To build it in release mode, issue the following command:

    cargo build --release

Whether you built Cred in debug or release mode, you will be able to find the build executable at`target/<mode>/cred`.

## Usage

Cred is currently a command line-only application. It has a self-documenting interface and you can see how to use it by running:

    cred help

## Development

So you’re interested in contributing to Cred? Check out our [contributing guidelines](CONTRIBUTING.md) for information on how to do that.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT.md) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache 2.0 license, shall be dual licensed as above, without any additional terms or conditions.
