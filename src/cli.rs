use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("call")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommands(vec![
            SubCommand::with_name("init")
                .about("init cargo call template at current working directory")
                .args(&[
                    Arg::with_name("name")
                        .default_value(".")
                        .help("Name of the project. Will create a new directory with that name in the current directory"),
                    Arg::with_name("force")
                        .short("f")
                        .takes_value(false)
                        .help("Force creation of template at current working directory")
                ]),
        ])
}
