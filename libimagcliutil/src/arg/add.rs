use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct AddArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for AddArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for AddArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for AddArgBuilder<'a> {
    fn default() -> AddArgBuilder<'a> {
        AddArgBuilder(ArgBuilder::new("add")
            .with_short("a")
            .with_long("add")
            .with_helptext("Add entry")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NAME"))
    }
}

impl<'a> AddArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("add")
    }
}

