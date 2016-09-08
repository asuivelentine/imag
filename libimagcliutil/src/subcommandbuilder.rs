use clap::{App, Arg, SubCommand};

pub struct SubCommandBuilder<'a> {
    name: &'a str,
    author: Option<&'a str>,
    about: Option<&'a str>,
    version: Option<&'a str>,
    usage: Option<&'a str>,
    arg: Option<Vec<&'a Arg<'a, 'a>>>,
}

impl<'a> SubCommandBuilder<'a> {

    pub fn new(name: &'a str) -> SubCommandBuilder<'a> {
        SubCommandBuilder{
            name:       name,
            author:     None,
            about:      None,
            version:    None,
            usage:      None,
            arg:        None,
        }
    }

    pub fn with_author(mut self, author: &'a str) -> SubCommandBuilder<'a> {
        self.author = Some(author);
        self
    }

    pub fn without_author(mut self) -> SubCommandBuilder<'a> {
        self.author = None;
        self
    }

    pub fn with_about(mut self, about: &'a str) -> SubCommandBuilder<'a> {
        self.about = Some(about);
        self
    }

    pub fn without_about(mut self) -> SubCommandBuilder<'a> {
        self.about = None;
        self
    }

    pub fn with_version(mut self, version: &'a str) -> SubCommandBuilder<'a> {
        self.version = Some(version);
        self
    }

    pub fn without_version(mut self) -> SubCommandBuilder<'a> {
        self.version = None;
        self
    }

    pub fn with_usage(mut self, usage: &'a str) -> SubCommandBuilder<'a> {
        self.usage = Some(usage);
        self
    }

    pub fn without_usage(mut self) -> SubCommandBuilder<'a> {
        self.usage = None;
        self
    }

    pub fn with_arg(mut self, arg: &'a Arg) -> SubCommandBuilder<'a> {
            if self.arg.is_none() {
                self.arg = Some(Vec::new());
            }

            self.arg = self.arg.and_then(|mut v| {
                v.push(arg);
                Some(v)
            });
            self
    }

    pub fn withouth_arg(mut self) -> SubCommandBuilder<'a> {
        self.arg = None;
        self
    }

    pub fn build<'b>(self) -> App<'a,'b> {
       let mut command = SubCommand::with_name(self.name);

       if let Some(a) = self.author {
            command = command.author(a);
       }

       if let Some(a) = self.about {
           command = command.about(a);
       }

       if let Some(v) = self.version {
           command = command.version(v);
       }

       if let Some(u) = self.usage {
           command = command.arg_from_usage(u);
       }

       command
    }

}

