use scraper::{Html, Selector};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader};
use std::path::Path;
use std::collections::BTreeMap;
use glob::glob;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Count {
    form: i32,
    controller: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Form_Data{
    action: String,
    method: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input_Data {
    kind_of_type: String,
    min: i32,
    max: i32,
}

fn print_typename<T>(_: T) {
    println!("type_name:{}", std::any::type_name::<T>());
}

fn write_model_file(node_and_arc: Count) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("test_model/data_json/model_file.json")?;
    println!("file:{:#?}",file);
    
    let mut counts = BTreeMap::new();
    counts.insert("parts",node_and_arc);

    // serialized
    let serialized: String = serde_json::to_string(&counts).unwrap();
    println!("counts:{}",&serialized);

    // write
    write!(&file, "{}", serialized)?;
    Ok(())
}

fn write_input_file(input_list : BTreeMap<String, BTreeMap<String, Input_Data>>)  -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("test_model/data_json/input_file.json")?;

    // serialized
    let serialized: String = serde_json::to_string(&input_list).unwrap();

    // write
    write!(&file, "{}", serialized)?;
    Ok(())
}

fn write_controller_name(controller_name_list : Vec<String>){
    let mut controller_file = File::create("test_model/data_txt/controller_name.txt").expect("file not found.");
    for controller_name in controller_name_list{
        let write_name_data = format!("{}",&controller_name);
        writeln!(controller_file, "{}", write_name_data).expect("cannot write.");
    }
}

// formの数と送り先などを記録予定
fn anarisis_html_form(file_name: &Path, form_list: &Vec<Form_Data>) -> i32{
    let mut html_file = match File::open(file_name){
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(html_file) => html_file,
    };
    
    let mut html_contents = String::new();
    match html_file.read_to_string(&mut html_contents){
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(_) => (),
    }

    let fragment = Html::parse_fragment(&html_contents);
    let selector = Selector::parse("form").unwrap();
    let mut count_form :i32 = 0;

    for element in fragment.select(&selector) {
        count_form+=1;
    }
    count_form
}

fn anarisis_html_input(file_name: &Path) -> BTreeMap<String, Input_Data>{
    let mut html_file = match File::open(file_name){
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(html_file) => html_file,
    };
    
    let mut html_contents = String::new();
    match html_file.read_to_string(&mut html_contents){
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(_) => (),
    }

    let fragment = Html::parse_fragment(&html_contents);
    let selector = Selector::parse("input").unwrap();
    let mut count_form :i32 = 0;
    let mut input_list = BTreeMap::new();

    for element in fragment.select(&selector) {
        let mut input_data = Input_Data{kind_of_type: "".to_string(), min: -1, max: -1,};
        let input_name = element.value().attr("name").unwrap().to_string();
        input_data.kind_of_type = element.value().attr("type").unwrap().to_string();
        // 属性が含まれているか判定
        if element.value().attr("minlength") != None {
            input_data.min = element.value().attr("minlength").unwrap().parse().unwrap();
        }
        else if element.value().attr("min") != None {
            input_data.min = element.value().attr("min").unwrap().parse().unwrap();
        }
        if element.value().attr("maxlength") != None {
            input_data.min = element.value().attr("maxlength").unwrap().parse().unwrap();
        }
        else if element.value().attr("max") != None {
            input_data.min = element.value().attr("max").unwrap().parse().unwrap();
        }
        input_list.insert(input_name, input_data);
    }
    input_list
}

fn anarisis_controller(file_name: &str) -> String{
    let file_name_len = &file_name.len();
    let first_pos = 16;
    let last_pos = file_name_len - 4;
    let mut controller_name = "".to_string();
    for (i,c) in file_name.chars().enumerate(){
        if i >= first_pos && i <= last_pos{
            controller_name.push(c);
        }
    }
    // println!("controller:{:?}",&controller_name);
    controller_name
}

fn main(){
    // let file_name = "./templates/login.html";
    // htmlファイルをリスト化
    let html_file_list = glob("./templates/view/*.html").unwrap()
    .map(|e| e.unwrap())
    .collect::<Vec<_>>();

    // controllerをリスト化
    let controller_file_list = glob("./src/controllers/*.rs").unwrap()
    .map(|e| e.unwrap())
    .collect::<Vec<_>>();

    
    let mut node_and_arc = Count{form:0, controller:-1};
    let mut input_list_json : BTreeMap<String, BTreeMap<String, Input_Data>> = BTreeMap::new();
    let mut form_list : Vec<Form_Data> = Vec::new();
    let mut controller_list : Vec<String> = Vec::new();

    for file_path in html_file_list{
        node_and_arc.form += anarisis_html_form(&file_path, &form_list);
        anarisis_html_input(&file_path);
        let input_list = anarisis_html_input(&file_path);
        let fname = file_path.file_name().unwrap().to_str().unwrap().to_string();
        input_list_json.insert(fname, input_list);
    }
    write_input_file(input_list_json);

    for file_path in controller_file_list{
        let file_name = file_path.to_str().unwrap();
        let controller_name = anarisis_controller(&file_name);
        match &*controller_name{
            "mod" => println!("mod!"),
            _ => controller_list.push(controller_name),
        }
        // println!("file:{:?}",file_name);
        node_and_arc.controller += 1;
    }
    // println!("controller_list:{:?}",controller_list);


    // let mut model_file: Vec<Count> = Vec::new();
    // model_file.push(node_and_arc);
    write_controller_name(controller_list);
    let result = write_model_file(node_and_arc);
    match result {
        Ok(..) => { println!("Write Finished") }
        Err(err) => { println!("Failed to Write: {}", err) }
    }

}