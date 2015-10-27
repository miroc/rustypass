use secstr::SecStr;

// The `derive` attribute automatically creates the implementation
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    title: String,
    username: String,
    password: SecStr
}

impl Entry {
    pub fn new<S: Into<String>>(title: S, username: S, password: S) -> Entry {
		Entry {
			title: title.into(),
			username: username.into(),
			password: SecStr::new(password.into()) // todo avoid cloning??
		}
	}
}

/* De/Serialization infrastructure */
// TODO replace with derived serialization
// impl Serialize for Entry {
//     fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
//         where S: Serializer
//     {
//         serializer.visit_struct("Entry", EntryMapVisitor {
//             value: self,
//             state: 0,
//         })
//     }
// }

// struct EntryMapVisitor<'a> {
//     value: &'a Entry,
//     state: u8,
// }
// impl<'a> MapVisitor for EntryMapVisitor<'a> {
//     fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
//         where S: Serializer
//     {
//         match self.state {
//             0 => {
//                 self.state += 1;
//                 Ok(Some(try!(serializer.visit_struct_elt("title", &self.value.title))))
//             }
//             1 => {
//                 self.state += 1;
//                 Ok(Some(try!(serializer.visit_struct_elt("username", &self.value.username))))
//             }
//             2 => {
//                 self.state += 1;
//                 Ok(Some(try!(serializer.visit_struct_elt("password", &self.value.password))))
//             }
//             _ => {
//                 Ok(None)
//             }
//         }
//     }
// }
//
//
