//! Contains GCC-specific implementations

use crate::compiler::command::CompilerCommandFlags;

pub const FLAGS: CompilerCommandFlags = CompilerCommandFlags {
    output: "-o",
    compile: "-c",
    include: "-I",
    language: "-x",
};