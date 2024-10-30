pub mod file_load;
pub mod query_file;
pub mod tags;
pub mod parsers;

use file_load::load_dir;
use query_file::QueryFileType;
use tags::Tag;

fn main() {
    let r = load_dir("./source-scripts");

    let mut tags: Vec<Tag> = vec![];

    for qf in r {
        match qf.file_type {
            QueryFileType::Lib => tags.extend(Tag::load_tags(qf.raw_contents)),
            QueryFileType::Proc => todo!(),
        }
    }
    println!("RESULTS");
    for tag in tags{
        if let Tag::Func(t) = tag{
            let s = t.sym;
            println!("{}",s.as_str());
            println!("{}", t.args.len());
            println!("{}", t.body);
        }
        else if let Tag::Struct(t) = tag {
            let s = t.sym;
            println!("{}",s.as_str());
            println!("{}", t.members.len());
        }
    }
}
