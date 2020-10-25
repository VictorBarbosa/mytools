use clap::{App, Arg};

#[path = "linux/command_line.rs"]
mod bs;
#[path = "windows/command_line.rs"]
mod cmd;
fn main() {
    let _app = App::new("mytools")
        .about("My tools are all my command lines to help me")
        .arg(
            Arg::with_name("node-delete")
                .short("d")
                .long("node-delete")
                .help("Find and delete all node_modules folder"),
        )
        .arg(
            Arg::with_name("What you looking for filename.extention")
                .alias("ali")
                .short("f")
                .long("find-file")
                .takes_value(true)
                .max_values(1)
                .help("Find a file on current directory or subdirectories")
                .use_delimiter(true),
        )
        .arg(
            Arg::with_name("The string you looking for")
                .alias("ali")
                .short("s")
                .long("find-string")
                .takes_value(true)
                .max_values(1)
                .help("Find a string inside all files on current directory or subdirectories")
                .use_delimiter(true),
        )
        .get_matches();

    let ret = _app.args.contains_key("node-delete");
    if ret {
        let _output = if cfg!(target_os = "windows") {
            let command = r#"FOR /d /r . %d in (node_modules) DO @if exist %d rd /s/q %d  && @ECHO  %d **DELETED**"#;
            cmd::command(command.to_string());
        } else {
            let commnad = r#"find . -type d -name "node_modules" -exec rm -rf {} +"#;
            bs::command(commnad.to_string())
        };
    }

    if let Some(_find) = _app.value_of("What you looking for filename.extention") {
        let _output = if cfg!(target_os = "windows") {
            let command = format!(r#"WHERE /r  .\  *{0}*"#, _find.to_string());
            cmd::command(command);
        } else {
        };
    }
    if let Some(_string) = _app.value_of("The string you looking for") {
        let _output = if cfg!(target_os = "windows") {
            let command = format!(r#"FINDSTR  /s  /m "amor" *.txt*"#);
            print!("{}", command.to_string());
            cmd::command(command);
        } else {
        };
    }
}
