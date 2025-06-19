# Copper - C/C++ build tool

Copper is an easily configurable build tool for C/C++ projects. It aims to work with most of the popular compilers 
(gcc, clang, msvc) while allowing quick and easy configuration for your project using a single `copper.toml` file.

> [!NOTE]
> Right now Copper is in a very early stage of development. Many of the described features might and will change in the 
> future.

## About

_I personally dislike CMake, Makefile seems hard to scale with large projects, and VS Solutions seems to verbose and 
clunky. So I decided to write my own build system to replace them in my projects. This project is mostly made for my 
own usage, but anyone else if welcomed to use it or contribute to the development._

Copper aims to be a simple tool which is configures using a single file, yet allowing to fully control build process 
of your C and C++ projects without manually writing compile, link and build logic. It automatically gathers sources and 
links them together according to the configuration, while also having options for additional parameters. Each Copper 
Project is made of smaller Copper Units which contain module-specific configuration and build rules.

**Please refer to the [Project Configuration File Reference](https://github.com/duckysmacky/copper/wiki/Project-Configuration#coppertoml-reference) 
in order to learn how to properly configure your Copper project**

## Installation

Since there is no official release yet, the only way to use Copper is to build it manually or install it directly from 
the lastest commit on the master branch:

```bash
cargo install --git https://github.com/duckysmacky/copper.git
```

## Usage

Copper works by reading project configuration defined in a `copper.toml` file. It should contain all the necessary 
information about source language, chosen compiler, additional flags, copper units, target directories, include paths, 
etc.

If you are familiar with Visual Studio and its Solution system, you can associate a Copper _Project_ with VS _Solution_ 
and Copper _Unit_ with VS _Project_.

#### Initialize a new Copper Project

```bash
copper init [location]
```

This will create a new copper project at the current location or a specified location with name. By default, it will 
also generate a basic example configuration to get easily started with (can be cancelled with `--minimal`)

#### Add a Copper Unit

```bash
copper new unit <source> <type>
```

This will add the specified `source` path as a Unit to the `copper.toml` file. `source` should a valid path to a 
directory containing source files, where all the files within are to be compiled and linked separately from the other 
Units. `type` is used to specify the output file type (this can be a `binary`, `static-library` or `dynamic-library`)

#### Build your Units and Project

```bash
copper build [unit]...
```

Build the whole project or only specified unit names. Will output binary, object and library files into directories 
specified in `copper.toml`.
