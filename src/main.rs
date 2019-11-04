use std::process::exit;
use crate::cli::{get_database_url, write_to_file};

mod cli;
mod db;

use db::mysql::connect;
use rustorm::{TableName, Table, Value};
use std::ops::Add;
use rustorm::types::SqlType;

#[derive(Debug)]
struct TableInfo {
    database_url: String,
    table_name: String,
}

impl TableInfo {
    fn get_database_url(&self) -> &String {
        &self.database_url
    }
    fn get_table_name(&self) -> &String {
        &self.table_name
    }
}

fn main() {
    let mut app = cli::create_clap();

    let matches = app.clone().get_matches();

    let mut table_info = None;

    match (&matches).subcommand() {
        ("table-name", Some(arg_matchs)) => {
            let database_url = get_database_url(&matches);
            let table_name = arg_matchs.value_of("table").unwrap_or_else(|| {
                println!("no table name");
                exit(-1)
            }).to_string();
            table_info = Some(TableInfo { database_url, table_name });
        }
        ("mybatis-xml", Some(arg_matchs)) => {
            let databaseurl = get_database_url(&matches);
        }
        (_, _) => {
            app.print_help().unwrap();
        }
    }

    table_info.map(|table| {
        let (mut pool, mut em) = connect(table.get_database_url().as_str());
        em.get_table(&TableName::from(table.get_table_name().as_str())).map(|table| {
            let s = table_to_string(table);
            write_to_file(&matches, s);
        }).unwrap_or_else(|e| {
            println!("{:?}", e);
        })
    }).unwrap_or_else(|| {})
}

fn table_to_string(table: Table) -> String {
    let mut s = format!("#[derive(Debug, FromDao, ToColumnNames )]\nstruct {}{}\n",
                        &table.name.safe_name(), "{");
    for col in &table.columns {
        s.push_str("\t");
        s.push_str(col.name.safe_complete_name().as_ref());
        s.push_str(" : ");
        if col.is_not_null(){
            s.push_str(sqltype_to_rusttype_string(&col.get_sql_type()).as_ref())
        }else {
            s.push_str(("Option<".to_string() + sqltype_to_rusttype_string(&col.get_sql_type()).as_ref()+">").as_ref())
        }
        s.push_str(",\n");
    }
    s.push_str("}");
    s
}

fn sqltype_to_rusttype_string(sqltype:&SqlType)->String{
    match sqltype {
        SqlType::Bool => {"bool".to_string()},
        SqlType::Tinyint => {"i8".to_string()},
        SqlType::Smallint => {"i16".to_string()},
        SqlType::Int => {"i32".to_string()},
        SqlType::Bigint => {"i64".to_string()},
        SqlType::Real => {"f8".to_string()},
        SqlType::Float => {"f32".to_string()},
        SqlType::Double => {"f64".to_string()},
        SqlType::Numeric => {"BigDecimal".to_string()},
        SqlType::Tinyblob => {"Vec<u8>".to_string()},
        SqlType::Mediumblob => {"Vec<u8>".to_string()},
        SqlType::Blob => {"Vec<u8>".to_string()},
        SqlType::Longblob => {"Vec<u8>".to_string()},
        SqlType::Varbinary => {"Vec<u8>".to_string()},
        SqlType::Char => {"String".to_string()},
        SqlType::Varchar => {"String".to_string()},
        SqlType::Tinytext => {"String".to_string()},
        SqlType::Mediumtext => {"String".to_string()},
        SqlType::Text => {"String".to_string()},
        SqlType::Json => {"String".to_string()},
        SqlType::TsVector => {"String".to_string()},
        SqlType::Uuid => {"Uuid".to_string()},
        SqlType::Date => {"NaiveDate".to_string()},
        SqlType::Timestamp => {"DateTime<Utc>".to_string()},
        SqlType::TimestampTz => {"DateTime<Utc>".to_string()},
        SqlType::Time => {"NaiveTime".to_string()},
        SqlType::TimeTz => {"NaiveTime".to_string()},
        SqlType::Interval => {"Interval".to_string()},
        SqlType::IpAddress => {"String".to_string()},
        SqlType::Point => {"Point<f64>".to_string()},
        SqlType::Enum(_, _) => {"Enum(String, Vec<String>)".to_string()},
        SqlType::Array(s) => {"Vec<".to_string()+ sqltype_to_rusttype_string(s).as_ref() +">"},
    }
}