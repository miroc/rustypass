use db::DatabaseInFile;

static USAGE: &'static str = "Invalid arguments.
Usage: rpass edit <entry>";

fn usage(){
    println!("{}", USAGE);
}

pub fn call(file_db: &mut Box<DatabaseInFile>, params: &[&str]){
    if params.len() == 0 {
        usage();
        return;
    }
    println!("Error: NOT IMPLEMENTED YET");
}
