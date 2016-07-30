use argbuilder::ArgBuilder;

pub struct DeleteArgBuilder<'a>(ArgBuilder<'a>);

impl<'a> DeleteArgBuilder<'a> {
    pub fn new() -> DeleteArgBuilder<'a> {
        DeleteArgBuilder(ArgBuilder::new("delete")
            .with_short("d")
            .with_long("delete")
            .with_helptext("deletes the given element")
            .with_takes_value(false)
            .with_required(false)
            .with_value_name("DELETE"))
    } 
}

