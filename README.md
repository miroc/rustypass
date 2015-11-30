# rustypass
Rustypass is a command line based password manager written in Rust. At this point, it is mainly a prototype done for fun.

## Description
* Rustypass uses NaCl library for its crypto, specifically TweetNacl implementation which is included in the project. 
* The code is small and easily auditable, including all the crypto implementation (not rolling any new crypto here).
* Database file contains:
  * DB version
  * Salt for Bcrypt
  * NaCl [Secretbox](http://nacl.cr.yp.to/secretbox.html) structure serialized to bytes, 
encrypted with password derived from master password using Bcrypt.
* Secretbox structure uses authenticated encryption (XSalsa20 + Poly1305) and contains database of entries serialized to JSON.
* Passwords are stored in SecStr structure, which keeps them encrypted in the memory (= obfuscation).

## Compilation
First, you need to install Rust with Cargo, see the [official page](https://www.rust-lang.org/downloads.html). Nightly build (tested on Nightly 1.6)
is required, because Serde JSON serialization library uses some of the nightly features.

To compile and run the project, go to the project directory and run:
```
cargo build --release
./target/release/rpass
```

## Basic usage
* `rpass create <db_filename>` - creates a new database
* `rpass open <db_filename>` - opens up an existing database

Once you have your DB opened, you can add, copy or remove entries. To see all the available commands, type `help`.

