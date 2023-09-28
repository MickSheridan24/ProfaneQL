pub mod file_load;
pub mod parsers;
pub mod query_file;
pub mod tags;

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

    for tag in tags{
        if let Tag::Func(t) = tag{
            let s = t.sym;
            print!("{}",s.as_str());
        }
    }
}
