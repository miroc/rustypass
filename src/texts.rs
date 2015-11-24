pub static USAGE: &'static str = "Usage: rpass [OPTIONS] COMMAND [arg...]

RustyPass is keeping your passwords safe, while:
    * being memory safe by default (unlike C/C++)
    * having no runtime (unlike java, C#, go)
    * no interpreted code (unlike python, ruby)

Commands:
    new \t Create new database
    open \t Open existing database

Options:
    -h, --help\t Show this help";

pub static DB_COMMANDS: &'static str = "Available commands:
    list \t List all entries
    new \t Add new entry
    show <entry> \t Show entry details
    copy <entry>\t Copy entry password to clipboard
    edit <entry> \t Edit entry details
    remove <entry>\t Remove entry";

//Tip: You don't have to type entry's full name, type only prefix and press TAB to autocomplete.";
