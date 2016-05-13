use std::cell::RefCell;

use rhai::{Engine, FnRegister};
use toml::Value;

use libimagstore::store::Entry;
use libimagstore::store::Result;
use libimagutil::trace::trace_error;

use filter::Filter;

pub struct RhaiFilter {
    source: String,
    engine: RefCell<Engine>,
}

impl RhaiFilter {

    pub fn new(source: String) -> RhaiFilter {
        let mut engine = Engine::new();

        engine.register_type::<RhaiEntry>();
        engine.register_fn("to_str",        RhaiEntry::to_str);
        engine.register_get("get_location", RhaiEntry::get_location);
        engine.register_get("get_content",  RhaiEntry::get_content);
        engine.register_get("get_header",   RhaiEntry::get_header);
        engine.register_fn("verify_header", RhaiEntry::verify_header);

        RhaiFilter {
            source: source,
            engine: RefCell::new(engine),
        }
    }

}

impl Filter for RhaiFilter {

    fn filter(&self, e: &Entry) -> bool {
        use rhai::Scope;

        let e = RhaiEntry::from(e);
        let mut scope: Scope = vec![(String::from("e"), Box::new(e))];
        self.engine.borrow_mut()
            .eval_with_scope::<bool>(&mut scope, &self.source[..])
            .map_err(|e| trace_error(&e))
            .unwrap_or(false)
    }
}

#[derive(Clone)]
struct RhaiEntry<'a>(&'a Entry);

impl<'a> RhaiEntry<'a> {

    fn to_str(&self) -> String {
        self.0.to_str()
    }

    fn get_location(&mut self) -> Option<String> {
        self.0.get_location().to_str().map(String::from)
    }

    fn get_content(&mut self) -> String {
        self.0.get_content().clone()
    }

    fn get_header(&mut self) -> Value {
        self.0.get_header().header().clone()
    }

    fn verify_header(&self) -> Result<()> {
        self.0.verify()
    }
}

impl<'a> From<&'a Entry> for RhaiEntry<'a> {

    fn from(e: &'a Entry) -> RhaiEntry<'a> {
        RhaiEntry(e)
    }

}
