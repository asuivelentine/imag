use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct SelectArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for SelectArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for SelectArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for SelectArgBuilder<'a> {
    fn default() -> SelectArgBuilder<'a> {
        SelectArgBuilder(ArgBuilder::new("select")
            .with_short("s")
            .with_long("select")
            .with_helptext("Select element")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("SELECT"))
    }
}

impl<'a> SelectArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches<'a>) -> bool {
        arg.is_present("select")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<&'a str> {
        arg.value_of("name")
    }
}

