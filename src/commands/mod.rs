use anyhow::Result;

use crate::config::Config;

macro_rules! re_export {
    ( $( $md:tt )+ ) => {
        $(
            mod $md;
            pub use $md::*;
        )*
    };
}

// List the names of your command modules to re-export them
// in this module.
re_export! {
    show_config
    fetch_sheet
}

pub trait Command {
    fn run(&self, config: &Config) -> Result<()>;
}

#[macro_export]
macro_rules! register_commands {
    ( $( $command:tt )+ ) => {
        #[derive(clap::Subcommand)]
        enum Commands {
            $(
                $command($command),
            )*
        }

        impl std::ops::Deref for Commands {
            type Target = dyn $crate::commands::Command;

            fn deref(&self) -> &Self::Target {
                match &self {
                    $(
                        Self::$command(c) => c,
                    )*
                }
            }
        }
    };
}
