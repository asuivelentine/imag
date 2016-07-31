use std::path::PathBuf;
use std::process::Command;
use std::env;
use std::io::stderr;
use std::io::Write;

pub use clap::App;

use clap::{Arg, ArgMatches};
use log;
use log::LogLevelFilter;

use configuration::Configuration;
use error::RuntimeError;
use error::RuntimeErrorKind;
use error::MapErrInto;
use logger::ImagLogger;

use libimagstore::store::Store;

#[derive(Debug)]
pub struct Runtime<'a> {
    rtp: PathBuf,
    configuration: Option<Configuration>,
    cli_matches: ArgMatches<'a>,
    store: Store,
}

impl<'a> Runtime<'a> {

    /**
     * Gets the CLI spec for the program and retreives the config file path (or uses the default on
     * in $HOME/.imag/config, $XDG_CONFIG_DIR/imag/config or from env("$IMAG_CONFIG")
     * and builds the Runtime object with it.
     *
     * The cli_spec object should be initially build with the ::get_default_cli_builder() function.
     *
     */
    pub fn new(cli_spec: App<'a, 'a>) -> Result<Runtime<'a>, RuntimeError> {
        use std::env;

        use libimagstore::hook::position::HookPosition as HP;
        use libimagstore::hook::Hook;
        use libimagstore::error::StoreErrorKind;
        use libimagstorestdhook::debug::DebugHook;
        use libimagerror::trace::trace_error;
        use libimagerror::trace::trace_error_dbg;
        use libimagerror::into::IntoError;

        use configuration::error::ConfigErrorKind;

        let matches = cli_spec.get_matches();

        let is_debugging = matches.is_present("debugging");
        let is_verbose   = matches.is_present("verbosity");

        Runtime::init_logger(is_debugging, is_verbose);

        let rtp : PathBuf = matches.value_of("runtimepath")
            .map_or_else(|| {
                env::var("HOME")
                    .map(PathBuf::from)
                    .map(|mut p| { p.push(".imag"); p})
                    .unwrap_or_else(|_| {
                        panic!("You seem to be $HOME-less. Please get a $HOME before using this software. We are sorry for you and hope you have some accommodation anyways.");
                    })
            }, PathBuf::from);
        let storepath = matches.value_of("storepath")
                                .map_or_else(|| {
                                    let mut spath = rtp.clone();
                                    spath.push("store");
                                    spath
                                }, PathBuf::from);

        let configpath = matches.value_of("config")
                                .map_or_else(|| rtp.clone(), PathBuf::from);

        let cfg = match Configuration::new(&configpath) {
            Err(e) => if e.err_type() != ConfigErrorKind::NoConfigFileFound {
                return Err(RuntimeErrorKind::Instantiate.into_error_with_cause(Box::new(e)));
            } else {
                warn!("No config file found.");
                warn!("Continuing without configuration file");
                None
            },

            Ok(mut cfg) => {
                if let Err(e) = cfg.override_config(get_override_specs(&matches)) {
                    error!("Could not apply config overrides");
                    trace_error(&e);

                    // TODO: continue question (interactive)
                }

                Some(cfg)
            }
        };

        let store_config = match cfg {
            Some(ref c) => c.store_config().cloned(),
            None        => None,
        };

        if is_debugging {
            write!(stderr(), "Config: {:?}\n", cfg).ok();
            write!(stderr(), "Store-config: {:?}\n", store_config).ok();
        }

        Store::new(storepath, store_config).map(|mut store| {
            // If we are debugging, generate hooks for all positions
            if is_debugging {
                let hooks : Vec<(Box<Hook>, &str, HP)> = vec![
                    (Box::new(DebugHook::new(HP::PreCreate))          , "debug", HP::PreCreate),
                    (Box::new(DebugHook::new(HP::PostCreate))         , "debug", HP::PostCreate),
                    (Box::new(DebugHook::new(HP::PreRetrieve))        , "debug", HP::PreRetrieve),
                    (Box::new(DebugHook::new(HP::PostRetrieve))       , "debug", HP::PostRetrieve),
                    (Box::new(DebugHook::new(HP::PreUpdate))          , "debug", HP::PreUpdate),
                    (Box::new(DebugHook::new(HP::PostUpdate))         , "debug", HP::PostUpdate),
                    (Box::new(DebugHook::new(HP::PreDelete))          , "debug", HP::PreDelete),
                    (Box::new(DebugHook::new(HP::PostDelete))         , "debug", HP::PostDelete),
                ];

                // If hook registration fails, trace the error and warn, but continue.
                for (hook, aspectname, position) in hooks {
                    if let Err(e) = store.register_hook(position, &String::from(aspectname), hook) {
                        if e.err_type() == StoreErrorKind::HookRegisterError {
                            trace_error_dbg(&e);
                            warn!("Registering debug hook with store failed");
                        } else {
                            trace_error(&e);
                        };
                    }
                }
            }

            Runtime {
                cli_matches: matches,
                configuration: cfg,
                rtp: rtp,
                store: store,
            }
        })
        .map_err_into(RuntimeErrorKind::Instantiate)
    }

    /**
     * Get a commandline-interface builder object from `clap`
     *
     * This commandline interface builder object already contains some predefined interface flags:
     *   * -v | --verbose for verbosity
     *   * --debug for debugging
     *   * -c <file> | --config <file> for alternative configuration file
     *   * -r <path> | --rtp <path> for alternative runtimepath
     *   * --store <path> for alternative store path
     * Each has the appropriate help text included.
     *
     * The `appname` shall be "imag-<command>".
     */
    pub fn get_default_cli_builder(appname: &'a str,
                                   version: &'a str,
                                   about: &'a str)
        -> App<'a, 'a>
    {
        App::new(appname)
            .version(version)
            .author("Matthias Beyer <mail@beyermatthias.de>")
            .about(about)
            .arg(Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .help("Enables verbosity")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("debugging")
                .long("debug")
                .help("Enables debugging output")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("config")
                .long("config")
                .help("Path to alternative config file")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("config-override")
                 .long("override-config")
                 .help("Override a configuration settings. Use 'key=value' pairs, where the key is a path in the TOML configuration. The value must be present in the configuration and be convertible to the type of the configuration setting. If the argument does not contain a '=', it gets ignored. Setting Arrays and Tables is not yet supported.")
                 .required(false)
                 .takes_value(true))

            .arg(Arg::with_name("runtimepath")
                .long("rtp")
                .help("Alternative runtimepath")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("storepath")
                .long("store")
                .help("Alternative storepath. Must be specified as full path, can be outside of the RTP")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("editor")
                .long("editor")
                .help("Set editor")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("term")
                 .long("term")
                 .help("set terminal emulator")
                 .required(false)
                 .takes_value(true))
    }

    /**
     * Initialize the internal logger
     */
    fn init_logger(is_debugging: bool, is_verbose: bool) {
        use std::env::var as env_var;
        use env_logger;

        if env_var("IMAG_LOG_ENV").is_ok() {
            env_logger::init().unwrap();
        } else {
            let lvl = if is_debugging {
                LogLevelFilter::Debug
            } else if is_verbose {
                LogLevelFilter::Info
            } else {
                LogLevelFilter::Warn
            };

            log::set_logger(|max_log_lvl| {
                max_log_lvl.set(lvl);
                debug!("Init logger with {}", lvl);
                Box::new(ImagLogger::new(lvl.to_log_level().unwrap()))
            })
            .map_err(|_| {
                panic!("Could not setup logger");
            })
            .ok();
        }
    }

    /**
     * Get the verbosity flag value
     */
    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbosity")
    }

    /**
     * Get the debugging flag value
     */
    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debugging")
    }

    /**
     * Get the runtimepath
     */
    pub fn rtp(&self) -> &PathBuf {
        &self.rtp
    }

    /**
     * Get the commandline interface matches
     */
    pub fn cli(&self) -> &ArgMatches {
        &self.cli_matches
    }

    /**
     * Get the configuration object
     */
    pub fn config(&self) -> Option<&Configuration> {
        self.configuration.as_ref()
    }

    /**
     * Get the store object
     */
    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn editor(&self) -> Option<Command> {
        self.cli()
            .value_of("editor")
            .map(String::from)
            .or({
                match self.configuration {
                    Some(ref c) => c.editor().cloned(),
                    _ => None,
                }
            })
            .or(env::var("EDITOR").ok())
            .map(Command::new)
    }

    pub fn terminal(&self) -> Option<Command> {
        self.cli()
            .value_of("term")
            .map(String::from)
            .or({
                match self.configuration {
                    Some(ref c) => c.term().cloned(),
                    _ => None,
                }
            })
            .or(env::var("TERM").ok()
                .or(Some("xterm".to_string())))
            .map(Command::new)
    }
}

fn get_override_specs(matches: &ArgMatches) -> Vec<String> {
    matches
        .values_of("config-override")
        .map(|values| {
             values
             .filter(|s| {
                 let b = s.contains("=");
                 if !b { warn!("override '{}' does not contain '=' - will be ignored!", s); }
                 b
             })
             .map(String::from)
             .collect()
        })
        .unwrap_or(vec![])
}

