use colored::Colorize;

pub struct Console;

impl Console {
    pub fn setup() {
        // need this for windows to enable ANSI :)
        colored::control::set_virtual_terminal(true).unwrap();

        println!(
            "{} v{}",
            r#"
             _           _ _   
            | |         | | |  
    ___ ___ | |__   __ _| | |_ 
   / __/ _ \| '_ \ / _` | | __|
  | (_| (_) | |_) | (_| | | |_ 
   \___\___/|_.__/ \__,_|_|\__|"#
                .bright_blue(),
            env!("CARGO_PKG_VERSION").bright_cyan()
        );

        println!("\nDownload URL: {}", "https://steele.gg/tools/cobalt".bright_green());

        println!("This tool will never cost money, if you paid you got {}", "scammed\n".red());

        println!(
            "{}",
            "Disclaimer\nAlthough this program has never had any reported bans it is a 3rd party tool and could \
             \npotentially be bannable. Use at your own risk I am not responsible for your account\n"
                .bright_red()
        );

        println!(
            "Cobalt now has a stealth mode if you let Cobalt start the league client no one will see you leave the lobby \
             :) (thanks to https://github.com/molenzwiebel/Deceive)"
        );

        println!("{}", "*** Hotkeys ***".bright_yellow());
        println!(
            "{} - Will dodge your current lobby (use this in the last 30 seconds of the lobby for best results)",
            "CTRL+D".bright_purple(),
        );
        println!(
            "{} - As long as you are under 195 RP this will get you a free battle boost (ARAM and other game modes)",
            "CTRL+B".bright_purple()
        );

        println!(
            "{} - This will restart your league client and enable stealth mode so no one is able to see you leave the \
             lobby (recommended not required)\n",
            "CTRL+R".bright_purple()
        );
    }
}
