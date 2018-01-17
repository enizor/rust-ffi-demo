# Fiddle is part of the standard Ruby library
require "fiddle"
require "fiddle/import"

module Rust
  extend Fiddle::Importer
  lib_ext = "dylib" if `uname` =~ /Darwin/
  lib_ext = "so" if `uname` =~ /Linux/
  dlload "./target/release/libffi_demo.#{lib_ext}"
  # the description of the function is, of-course, using C types
  extern 'int nb_of_perfect(int)'
end

# simply call the library
puts Rust.nb_of_perfect(ARGV[0].to_i)
