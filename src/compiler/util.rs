//! Utility module containing miscellaneous functions related to compiler

use std::env;
use std::path::PathBuf;
use crate::config::project::ProjectCompiler;

// TODO: improve logic, add better availability feedback
/// A basic function to check weather the compiler is available to use (checks if the system has the
/// compiler executable in $PATH).
pub fn check_if_available(compiler: &ProjectCompiler) -> bool {
    match compiler {
        ProjectCompiler::GCC => find_executable(compiler.get_executable()).is_some(),
        ProjectCompiler::GPP => find_executable(compiler.get_executable()).is_some(),
        ProjectCompiler::CLANG => find_executable(compiler.get_executable()).is_some(),
        ProjectCompiler::MSVC => {
            if !cfg!(windows) { return false; }

            if let None = find_executable(compiler.get_executable()) {
                return false;
            }

            // Additional program for linking (used to separately link DLLs and such)
            if let None = find_executable("link".to_string()) {
                return false;
            }

            true
        }
    }
}

/// Searches the system's $PATH environment variable for the matching executable name
fn find_executable(executable_name: String) -> Option<PathBuf> {
    let mut executable_name = PathBuf::from(executable_name);

    // Should work in 90% of the cases for Windows
    if cfg!(windows) {
        executable_name.set_extension("exe");
    }

    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let executable_path = dir.join(&executable_name);

            if executable_path.is_file() {
                Some(executable_path)
            } else {
                None
            }
        }).next()
    })
}