pub mod file_load;
pub mod query_file;
pub mod parsers;
pub mod file_process;
pub mod transcriber;
pub mod tokenizer;

use file_load::load_dir;
use file_process::process_files;
use parsers::file_parse::types::FileParseState;




fn main() {
    let r = load_dir("./source-scripts");


    let library = process_files(r);

    println!("Structs");
    for s in library.structs{
        println!("- {0}", s.name);
    }
    println!("Funcs");
    for s in library.funcs{
        println!("- {0}", s.name);
    }
}
