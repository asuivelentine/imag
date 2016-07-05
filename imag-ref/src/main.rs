#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate semver;
extern crate clap;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagref;
extern crate libimagerror;
extern crate libimagentrylist;

mod ui;
use ui::build_ui;

use std::path::PathBuf;

use libimagref::reference::Ref;
use libimagref::flags::RefFlags;
use libimagerror::trace::trace_error;
use libimagrt::setup::generate_runtime_setup;
use libimagrt::runtime::Runtime;

fn main() {
    let rt = generate_runtime_setup("imag-ref",
                                    &version!()[..],
                                    "Reference files outside of the store",
                                    build_ui);
    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "add"    => add(&rt),
                "remove" => remove(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn add(rt: &Runtime) {
    let cmd  = rt.cli().subcommand_matches("add").unwrap();
    let path = cmd.value_of("path").map(PathBuf::from).unwrap(); // saved by clap

    let flags = RefFlags::default()
        .with_content_hashing(cmd.is_present("track-content"))
        .with_permission_tracking(cmd.is_present("track-permissions"));

    Ref::create(rt.store(), path, flags)
        .map(|r| debug!("Reference created: {:?}", r))
        .map(|_| info!("Ok"))
        .map_err(|e| {
            trace_error(&e);
            warn!("Failed to create reference");
        });
}

fn remove(rt: &Runtime) {
    unimplemented!()
}

fn list(rt: &Runtime) {
    unimplemented!()
}

