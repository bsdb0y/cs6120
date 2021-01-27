#[macro_use]
extern crate generator;
use generator::Gn;

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
            for block in basic_block(item) {
                println!("{:?}", block);
            }
        }
    }
}
