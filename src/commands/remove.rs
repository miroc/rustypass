use db::DatabaseInFile;

static USAGE: &'static str = "Invalid arguments.
Usage: rpass remove <entry>";

fn usage(){
    println!("{}", USAGE);
}

pub fn call(file_db: &mut Box<DatabaseInFile>, params: &[&str]){
    if params.len() == 0 {
        usage();
        return;
    }

    let entry_id = params[0];
    if file_db.db.remove(entry_id){
        println!("Entry '{}' was successfully removed.", entry_id);
    } else {
        println!("No entry named '{}' was found.", entry_id);
    }
}
