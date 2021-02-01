#[macro_use]
extern crate generator;
use generator::Gn;
use indexmap::IndexMap;

fn block_map<'a>(blocks: generator::Generator<'_, (), Vec<&'a serde_json::Value>>) -> IndexMap<String, Vec<&'a serde_json::Value>> {
    let mut hashmap : IndexMap<String, Vec<&serde_json::Value>> =  IndexMap::new();
    let mut name: String;
    for block in blocks {
        if block[0]["label"] != serde_json::json!(null) {
            name = block[0]["label"].as_str().unwrap().to_string();
        }
        else {
            name = format!("b{}", hashmap.len().to_string());
        }
        hashmap.insert(name, block);
    }
    println!("{:?}", hashmap);
    hashmap
}

fn basic_block(body: &serde_json::Value) -> generator::Generator<'_,(), Vec<&serde_json::Value>> {
    let mut temp: Vec<&serde_json::Value> = vec![];
    let terminators: Vec<_> = vec!["jmp", "br", "ret"]
        .into_iter()
        .map(String::from)
        .collect();
    let gen_ret = Gn::new_scoped(move |mut s| {
        for instrs in body["instrs"].as_array() {
            for item in instrs {
                if  item["op"] != serde_json::json!(null) {
                    temp.push(item);
                    if terminators.contains(&(item["op"].as_str().unwrap().to_string())) {
                        s.yield_(temp);
                        temp = vec![];
                    }
                }
                else {
                    if temp.len() != 0 {
                        s.yield_(temp);
                    }
                    temp = vec![item];
                }
            }
        }
        if temp.len() != 0 {
            s.yield_(temp);
        }
        done!();
    });
    gen_ret
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} <json_file>", &args[0]); 
        std::process::exit(-1);
    }
    let filename: &String = &args[1];
    let file = std::fs::File::open(filename)
        .expect("file should open read only");
    let json_obj: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    for items in json_obj["functions"].as_array() {
        for item in items.iter() {
            block_map(basic_block(item));
        }
    }
}
