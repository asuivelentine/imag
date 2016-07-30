use argbuilder::ArgBuilder;

pub struct DeleteArgBuilder;

impl<'a> DeleteArgBuilder {
    pub fn new() -> ArgBuilder<'a> {
        ArgBuilder::new("delete")
            .with_short("d")
            .with_long("delete")
            .with_helptext("deletes the given element")
            .with_takes_value(false)
            .with_required(false)
            .with_value_name("DELETE")
    } 
}
