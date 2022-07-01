use std::fs;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, Write};

fn main() {
    println!("generating rust types from protos...");

    let rust_protos = fs::read_dir(env!("OUT_DIR")).unwrap();

    for rust_proto in rust_protos {
        let file_name = rust_proto
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .replace(".rs", "")
            .replace(".", "_")
            + ".rs";
        println!("{}", file_name);

        let file_path = rust_proto
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .replace(".rs", "")
            .replace(".", "/");

        let file_contents =
            BufReader::new(File::open(rust_proto.as_ref().unwrap().path()).unwrap()).lines();
        let mut new_file_contents = vec![];
        for line in file_contents {
            let mut l = line.unwrap();
            if let Some(idx) = l.find("super::") {
                l.insert_str(idx, "super::");
            }
            new_file_contents.push(l);
        }
        let new_file_contents = new_file_contents.join("\n");
        let mut new_file = File::create(rust_proto.as_ref().unwrap().path()).unwrap();
        new_file.write(new_file_contents.as_ref()).unwrap();

        rename(
            [
                env!("OUT_DIR"),
                "/",
                rust_proto.unwrap().file_name().to_str().unwrap(),
            ]
            .concat(),
            ["src/", &file_path, "/", &file_name].concat(),
        )
        .unwrap();
    }

    println!("fin");
}
