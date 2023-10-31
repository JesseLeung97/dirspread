use crate::{terminals::terminal_interface::TerminalInterface, Dirspread};

#[macro_export]
macro_rules! terminal {
    ( $( $x:expr ),* ) => {
        {
            let mut cmd = std::process::Command::new("osascript");
            $(
                cmd.arg($x);
            )*
            cmd
        }
    };
}

pub struct Macos {}
impl TerminalInterface for Macos {
    fn open_terminals(dirspread: &crate::Dirspread) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = terminal!(
            "-e", 
            "tell application \"Terminal\" to activate", 
            "-e", 
            "tell application \"System Events\" to tell process \"Terminal\" to keystroke \"n\"n using command down"
        );
        cmd.output()?;

        if let Some(win_name) = &dirspread.win_name {
            let win_name = win_name.to_owned();
            let mut cmd = terminal!(
                "-e", 
                format!("tell application \"Terminal\" to set custom title of front window to \"{win_name}\"")
            );
            cmd.output()?;
        }

        if let Some(dirs) = &dirspread.dirs {
            for dir in dirs {
                let dir_name = dir.dir_name.to_owned();
                if let Some(dir_path) = Dirspread::get_full_path(dir_name, &dirspread.parent_dir) {
                    let mut cmd = terminal!(
                        "-e", 
                        "tell application \"System Events\" to tell process \"Terminal\" to keystroke \"t\" using command down"
                    );
                    cmd.output()?;
                    let mut cmd = terminal!(
                        "-e",
                        format!("tell application \"Terminal\" to do script \"cd {dir_path}\" in selected tab of front window")
                    );
                    cmd.output()?;

                    // Set display name if exists
                    if let Some(disp_name) = &dir.disp_name {
                        let disp_name = disp_name.to_owned();
                        let mut cmd = terminal!(
                            "-e",
                            format!("tell application \"Terminal\" to set custom title of selected tab of front window to \"{disp_name}\"")
                        );
                        cmd.output()?;
                    }

                    // Run the on_open command
                    if let Some(on_open) = &dir.on_open {
                        let on_open = on_open.to_owned();
                        let mut cmd = terminal!(
                            "-e",
                            format!("tell application \"Terminal\" to do script \"{on_open}\" in selected tab of front window")
                        );
                        cmd.output()?;
                    }
                }
            }
        }

        // Close template tab
        let mut cmd = terminal!(
            "-e",
            "tell application \"System Events\" to tell process \"Terminal\" to keystroke (ASCII character 9) using control down"
        );
        cmd.output()?;
        let mut cmd = terminal!(
            "-e",
            "tell application \"System Events\" to tell process \"Terminal\" to keystroke \"w\" using command down"
        );
        cmd.output()?;
        
        Ok(())
    }
}
