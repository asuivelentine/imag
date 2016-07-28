use std::option::Option;

pub struct ArgBuilder<'a> {
    name: &'a str,
    long: Option<&'a str>,
    short: Option<&'a str>,
    takes_value: Option<bool>,
    required: Option<bool>,
    helptext: Option<&'a str>,
    value_name: Option<&'a str>,
}

impl<'a> ArgBuilder<'a> {
    pub fn new(name: &'a str) -> ArgBuilder<'a> {
        ArgBuilder{
            name:           name,
            long:           None,
            short:          None,
            takes_value:    None,
            required:       None,
            helptext:       None,
            value_name:     None,
        }
    }

    pub fn with_long(mut self, long: &'a str) -> ArgBuilder<'a> {
        self.long = Some(long);
        self
    }

    pub fn without_long(mut self) -> ArgBuilder<'a> {
        self.long = None;
        self
    }

    pub fn with_short(mut self, short: &'a str) -> ArgBuilder<'a> {
        self.short = Some(short);
        self
    }

    pub fn without_short(mut self) -> ArgBuilder<'a> {
        self.short = None;
        self
    }

    pub fn with_takes_value(mut self, takes_value: bool) -> ArgBuilder<'a> {
        self.takes_value = Some(takes_value);
        self
    }

    pub fn without_takes_value(mut self) -> ArgBuilder<'a> {
        self.takes_value = None;
        self
    }

    pub fn with_required(mut self, required: bool) -> ArgBuilder<'a> {
        self.required = Some(required);
        self
    }

    pub fn without_required(mut self) -> ArgBuilder<'a> {
        self.required = None;
        self
    }

    pub fn with_helptext(mut self, helptext: &'a str) -> ArgBuilder<'a> {
        self.helptext = Some(helptext);
        self
    }

    pub fn without_helptext(mut self) -> ArgBuilder<'a> {
        self.helptext = None;
        self
    }

    pub fn with_value_name(mut self, value_name: &'a str) -> ArgBuilder<'a> {
        self.value_name = Some(value_name);
        self
    }

    pub fn without_value_name(mut self) -> ArgBuilder<'a> {
        self.value_name = None;
        self
    }
}
