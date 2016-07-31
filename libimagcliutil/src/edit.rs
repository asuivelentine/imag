use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct EditArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for EditArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for EditArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for EditArgBuilder<'a> {
    fn default() -> EditArgBuilder<'a> {
        EditArgBuilder(ArgBuilder::new("edit")
            .with_short("e")
            .with_long("edit")
            .with_helptext("edit the given element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("EDIT"))
    }
}

impl<'a> EditArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("edit")
    }
}

