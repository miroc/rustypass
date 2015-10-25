#![feature(custom_derive, plugin)]
#[allow(dead_code)]
//#![plugin(serde_macros)]

use secstr::SecStr;

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// #[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    title: String,
    username: String,
    password: SecStr
}

impl Entry {
    //&str. This is a reference to another string
	// no self -- associated function
	//fn new(title: &str, username: &str, password: &str) -> PassEntry {
    pub fn new<S: Into<String>>(title: S, username: S, password: &String) -> Entry {
        //let () = password;
		Entry {
			title: title.into(),
			username: username.into(),
			password: SecStr::new(password.clone()) // todo avoid cloning??
		}
	}
}
