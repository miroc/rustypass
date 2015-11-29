use secstr::SecStr;

// The `derive` attribute automatically creates the implementation
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub title: String,
    username: String,
    password: SecStr
}

impl Entry {
    pub fn new<S: Into<String>>(title: S, username: S, password: S) -> Entry {
		Entry {
			title: title.into(),
			username: username.into(),
			password: SecStr::new(password.into())
		}
	}

    pub fn print_short_info_desc(){
        macro_rules! row {() => ("{0: <10} | {1: <10}")};
        println!(row!(), "Title", "Username");
    }

    pub fn print_short_info(&self){
        macro_rules! row {() => ("{0: <10} | {1: <10}")};
        println!(row!(), self.title, self.username);
    }

    pub fn print_full_info(&self){
        macro_rules! row {() => ("{0: <10} {1: <10}")};

        println!(row!(), "Title:", self.title);
        println!(row!(), "Username:", self.username);
        println!(row!(), "Password:", self.password);
    }

    pub fn copy_pass_to_clipboard(&self){
        println!("Error: NOT IMPLEMENTED YET");
    }
}
