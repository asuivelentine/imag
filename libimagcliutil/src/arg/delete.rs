use argbuilder::ArgBuilder;

use std::ops::Deref;
use std::ops::DerefMut;
use std::default::Default;

use clap::ArgMatches;

pub struct DeleteArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> Deref for DeleteArgBuilder<'a> {
    type Target = ArgBuilder<'a>;

    fn deref(&self) -> &ArgBuilder<'a> {
        &self.0
    }
}

impl<'b> DerefMut for DeleteArgBuilder<'b> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut ArgBuilder<'b> {
        &mut self.0
    }
}

impl<'a> Default for DeleteArgBuilder<'a> {

    /// Builds a default DeleteArgBuilder with
    ///
    ///  * name = "delete"
    ///  * short = "d"
    ///  * long = "delete"
    ///  * helptext = "deletes the given element"
    ///  * takes_value = false
    ///  * required = false
    ///  * ut_value_name =
    ///
    fn default() -> DeleteArgBuilder<'a> {
        DeleteArgBuilder(ArgBuilder::new("delete")
            .with_short("d")
            .with_long("delete")
            .with_helptext("deletes the given element")
            .with_takes_value(false)
            .with_required(false)
            .without_value_name())
    }
}

impl<'a> DeleteArgBuilder<'a> {
    pub fn arg_present(&self, arg: ArgMatches) -> bool {
        arg.is_present("delete")
    }
}

