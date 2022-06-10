use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]

struct Config {
    databases: BTreeMap <String, BTreeMap <String, BTreeMap <String, String>>>
}

fn read_json() -> BTreeMap <String, BTreeMap <String, BTreeMap <String, String>>>{
    // fn read_json(){
        let json_file_name = "./model/databases.json";
        let json_file = File::open(json_file_name).unwrap();
        let reader_json = BufReader::new(json_file);
        let _config_json : Config = serde_json::from_reader(reader_json).unwrap();
        println!("{:?}",_config_json);
        _config_json.databases
    }
    
fn connection_DB_and_create_Table(db_name : &String, create_table : String) -> Result<()>{
    let cn = Connection::open(db_name)?;
    cn.execute(&create_table, params![])?;
    println!("Create tables.");
    // println!("{:?}",create_table);
    Ok(())
}

fn process_json(config_json_databases : BTreeMap <String, BTreeMap <String, BTreeMap<String,String>>>) {
    for (sql,tables) in config_json_databases.iter() {
        let db_name = format!("./model/db/{}.db", sql);
        for (table, colums) in tables{
            let mut create_table = format!("CREATE TABLE {} (", table);
            for (colum,type_colum) in colums{
                create_table = format!("{}{} {},", create_table, colum, type_colum); 
            }
            create_table.pop();
            create_table = create_table + ")";
            // println!("{:?}",create_table);
            connection_DB_and_create_Table(&db_name, create_table);
        }
    }
}

fn main() {
    let config_json_databases;
    println!("Hello World!");
    config_json_databases = read_json();
    process_json(config_json_databases);
}