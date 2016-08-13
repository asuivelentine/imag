macro_rules! build_subcommand_builder {
    {
        $typename: ident,
        $modname: ident,
        $argname: expr,
        $defaultimpl: expr
    } => {
        mod $modname {
            use subcommandbuilder::SubCommandBuilder;

            use std::ops::Deref;
            use std::ops::DerefMut;
            use std::default::Default;

            pub struct $typename<'a>(SubCommandBuilder<'a>);

            impl<'a> Deref for $typename<'a> {
                type Target = SubCommandBuilder<'a>;

                fn deref(&self) -> &SubCommandBuilder<'a> {
                    &self.0
                }
            }

            impl<'b> DerefMut for $typename<'b> {
                fn deref_mut<'a>(&'a mut self) -> &'a mut SubCommandBuilder<'b> {
                    &mut self.0
                }
            }

            impl<'a> Default for $typename<'a> {

                fn default() -> $typename<'a> {
                    $typename($defaultimpl(SubCommandBuilder::new($argname)))
                }
            }
            
        }
    }

}

