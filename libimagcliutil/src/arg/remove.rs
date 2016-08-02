use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct RemoveArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for RemoveArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for RemoveArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for RemoveArgBuilder<'a> {
    fn default() -> RemoveArgBuilder<'a> {
        RemoveArgBuilder(ArgBuilder::new("remove")
            .with_short("r")
            .with_long("remove")
            .with_helptext("remove the Element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("REMOVE"))
    }
}

impl<'a> RemoveArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("remove")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<&'a str> {
        arg.value_of("remove")
    }
}

