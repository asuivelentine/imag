# libimagcliutil

A utility library for building commandline interfaces with `clap`.

The problem this library tries to solve is that the commandline interface has to
be consistent over crates. This library provides some defaults for commandline
interface types (mainly `clap::Arg` and `clap::SubCommand`) which can be used to
build the commandline interface of a imag module.

This library helps with code duplication while still remaining flexible in the
UI interface specification.

