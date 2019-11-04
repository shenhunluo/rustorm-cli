use clap::{Arg, App, SubCommand, ArgMatches};
use std::process::exit;
use std::fs::File;
use std::io::{Write, ErrorKind};

pub fn create_clap() -> App<'static, 'static> {
    App::new("rustorm-cli")
        .version("0.1")
        .author("shenhunluo")
        .about("get struct for rustorm")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("verbosity level"))
        .args_from_usage("-d --database-url=[url] 'database url'")
        .args_from_usage("-o --output=[fileName] 'output file'")
        .subcommand(
            SubCommand::with_name("mybatis-xml")
                .args_from_usage("-f --file=[fileName] 'mybatis xml file name'")
                .about("get struct by mybatis-xml")
        )
        .subcommand(
            SubCommand::with_name("table-name")
                .args_from_usage("-t --table=[tableName] 'table name'")
                .about("get struct by table name")
        )
}

pub fn get_database_url(matches: &ArgMatches) -> String {
    matches
        .value_of("database-url")
        .map(|s| s.into())
        .or_else(|| dotenv::var("DATABASE_URL").ok()).unwrap_or_else(|| {
        println!("no database-url");
        exit(-1);
    })
}

pub fn write_to_file(matches: &ArgMatches, content: String) {
    matches
        .value_of("output")
        .map(|file_name| {
            File::create(file_name).map(|mut file| {
                let mut p = file.metadata().unwrap().permissions();
                p.set_readonly(false);
                file.set_permissions(p).unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }).unwrap_or_else(|e| {
                println!("{:?}", &e);
                exit(-202);
            })
        })
        .unwrap_or_else(|| {
            println!("{}", content);
        });
}