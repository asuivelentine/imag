use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct NameArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for NameArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for NameArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for NameArgBuilder<'a> {

    /// Builds a default NameArgBuilder with
    ///
    ///  * name        = "name"
    ///  * short       = "n"
    ///  * long        = "name"
    ///  * helptext    = "select element by name"
    ///  * takes_value = true
    ///  * required    = false
    ///  * value_name  = "NAME"
    ///
    fn default() -> NameArgBuilder<'a> {
        NameArgBuilder(ArgBuilder::new("name")
            .with_short("n")
            .with_long("name")
            .with_helptext("select element by name")
            .with_takes_value(true)
            .with_required(false)
            .with_value_name("NAME"))
    }
}

impl<'a> NameArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("name")
    }

    pub fn fetch_value(&self, arg: &'a ArgMatches) -> Option<&'a str> {
        arg.value_of("name")
    }
}

