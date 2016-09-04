macro_rules! build_arg_builder {
    {
        $typename: ident,
        $modname: ident,
        $argname: expr,
        $defaultimpl: expr
    } => {
        mod $modname {
            use $crate::argbuilder::ArgBuilder;

            use std::ops::Deref;
            use std::ops::DerefMut;
            use std::default::Default;

            use clap::ArgMatches;

            pub struct $typename<'a>(ArgBuilder<'a>);

            impl<'a> Deref for $typename<'a> {
                type Target = ArgBuilder<'a>;

                fn deref(&self) -> &ArgBuilder<'a> {
                    &self.0
                }
            }

            impl<'b> DerefMut for $typename<'b> {
                fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
                    &mut self.0
                }
            }

            impl<'a> Default for $typename<'a> {

                fn default() -> $typename<'a> {
                    $typename($defaultimpl(ArgBuilder::new($argname)))
                }
            }

            impl<'a> $typename<'a> {
                pub fn arg_present(&self, arg: ArgMatches) -> bool {
                    arg.is_present($argname)
                }
            }

        }
    }

}

pub mod add;
pub mod create;
pub mod daterange;
pub mod delete;
pub mod edit;
pub mod name;
pub mod noedit;
pub mod range;
pub mod remove;
pub mod select;
pub mod timerange;
pub mod yes;
