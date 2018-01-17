After installing rust
`curl https://sh.rustup.rs -sSf | sh`

Compile using
`cargo build --release`

Test the ruby version vs the FFI usage with
`time ruby ruby.rb 1000`
`time ruby ffi.rb 1000`

Want to check the unit tests?
`cargo test`
