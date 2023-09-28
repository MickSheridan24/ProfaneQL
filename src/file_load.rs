use std::{
    env,
    fs::{self, metadata},
};

use crate::query_file::QueryFile;

pub fn load_dir(init_path: &str) -> Vec<QueryFile> {
    let mut ret = vec![];

    let res = env::current_dir().unwrap();
    let cwd = res.into_os_string().into_string().unwrap();
    print!("{}", cwd);
    let paths = fs::read_dir(init_path).unwrap();
    for path in paths {
        let pathstr = path.unwrap().path().into_os_string().into_string().unwrap();
        let md = metadata(pathstr.clone()).unwrap();

        if md.is_file() && pathstr.to_string().ends_with(".proql") {
            println!("{}", pathstr.clone());

            let file = fs::read_to_string(pathstr.clone())
                .expect("Should have been able to read the file {pathstr}");

            let qf = QueryFile::create(pathstr, file.split("\n").map(|r| r.to_string()).collect());

            ret.push(qf);
        } else if md.is_dir() {
            let sub_files = load_dir(&pathstr);

            if sub_files.len() > 0 {
                ret.extend(sub_files);
            }
        }
    }
    ret
}
