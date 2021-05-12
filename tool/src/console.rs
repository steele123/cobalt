use colored::Colorize;

pub struct Console;

impl Console {
    pub fn setup() {
        // need this for windows to enable ANSI :)
        colored::control::set_virtual_terminal(true).unwrap();

        println!(
            "{}",
            r#"
           _           _ _   
          | |         | | |  
  ___ ___ | |__   __ _| | |_ 
 / __/ _ \| '_ \ / _` | | __|
| (_| (_) | |_) | (_| | | |_ 
 \___\___/|_.__/ \__,_|_|\__|                                  
"#
            .bright_blue()
        );

        println!("{} {}", "\nDownload URL:", "https://steele.gg/tools/cobalt".bright_green());

        println!("This tool will never cost money, if you paid you got {}", "scammed\n".red());

        println!("{}", "*** Hotkeys ***".bright_yellow());
        println!(
            "{} - {}",
            "CTRL+D".bright_purple(),
            "Will dodge your current lobby (use this in the last 30 seconds of the lobby for best results)"
        );
        println!(
            "{} - {}",
            "CTRL+B".bright_purple(),
            "As long as you are under 195 RP this will get you a free battle boost (ARAM and other gamemodes)\n"
        );
    }
}
