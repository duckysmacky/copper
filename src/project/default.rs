use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Contains default values for project configuration
pub mod values {
    #![allow(dead_code)]
    use std::path::PathBuf;
    use paste::paste;

    /// Macro for generating default value constant lambdas and comparison functions for serde to use
    /// them in its `default` and `skip_serializing_if` attributes. Serde requires arguments to these
    /// attributes to be functions, so this macro helps generate those functions without much
    /// boilerplate.
    macro_rules! default_value {
        ($name:ident: $type_:ident = $value:expr) => {
            pub const $name: fn() -> $type_ = || $value;
            paste! {
                pub const [<IS_ $name>]: fn(& $type_) -> bool = |val: & $type_| val.eq(&$name());
            }
        };
    }

    default_value!(SOURCE_DIRECTORY_NAME: PathBuf = PathBuf::from("src"));
    default_value!(BUILD_DIRECTORY_NAME: PathBuf = PathBuf::from("build"));
    default_value!(BINARY_DIRECTORY_NAME: PathBuf = PathBuf::from("bin"));
    default_value!(LIBRARY_DIRECTORY_NAME: PathBuf = PathBuf::from("lib"));
    default_value!(OBJECT_DIRECTORY_NAME: PathBuf = PathBuf::from("obj"));

    default_value!(BINARY_DIRECTORY: PathBuf = BUILD_DIRECTORY_NAME().join(BINARY_DIRECTORY_NAME()));
    default_value!(LIBRARY_DIRECTORY: PathBuf = BUILD_DIRECTORY_NAME().join(LIBRARY_DIRECTORY_NAME()));
    default_value!(OBJECT_DIRECTORY: PathBuf = BUILD_DIRECTORY_NAME().join(OBJECT_DIRECTORY_NAME()));
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectDefaults {
    /// Default binary directory path for storing built binaries (executables)
    #[serde(default = "values::BINARY_DIRECTORY")]
    #[serde(skip_serializing_if = "values::IS_BINARY_DIRECTORY")]
    pub binary_directory: PathBuf,
    /// Default library directory path for storing built libraries (static and dynamic)
    #[serde(default = "values::LIBRARY_DIRECTORY")]
    #[serde(skip_serializing_if = "values::IS_LIBRARY_DIRECTORY")]
    pub library_directory: PathBuf,
    /// Default object files directory path for storing temporary object files
    #[serde(default = "values::OBJECT_DIRECTORY")]
    #[serde(skip_serializing_if = "values::IS_OBJECT_DIRECTORY")]
    pub object_directory: PathBuf,
}

impl ProjectDefaults {
    /// Checks if all values are set to their default values
    pub fn all_default(&self) -> bool {
        values::IS_BINARY_DIRECTORY(&self.binary_directory) &&
        values::IS_LIBRARY_DIRECTORY(&self.library_directory) &&
        values::IS_OBJECT_DIRECTORY(&self.object_directory)
    }
}

impl Default for ProjectDefaults {
    fn default() -> Self {
        ProjectDefaults {
            binary_directory: values::BINARY_DIRECTORY(),
            library_directory: values::LIBRARY_DIRECTORY(),
            object_directory: values::OBJECT_DIRECTORY(),
        }
    }
}