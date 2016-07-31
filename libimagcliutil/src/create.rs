use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct CreateArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for CreateArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for CreateArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for CreateArgBuilder<'a> {
    fn default() -> CreateArgBuilder<'a> {
        CreateArgBuilder(ArgBuilder::new("create")
            .with_short("c")
            .with_long("create")
            .with_helptext("creates a new element with name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("CREATE"))
    }
}

impl<'a> CreateArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("create")
    }
}

