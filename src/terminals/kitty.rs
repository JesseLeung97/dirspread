use crate::{terminals::terminal_interface::TerminalInterface, Dirspread};

#[macro_export]
macro_rules! kitty {
    ($( $x:expr ),* ) => {
        {
            let mut cmd = std::process::Command::new("kitty");
            $(
                cmd.arg($x);
            )*
            cmd
        }
    };
}

pub struct Kitty {}
impl TerminalInterface for Kitty {
    fn open_terminals(dirspread: &Dirspread) -> Result<(), Box<dyn std::error::Error>> {
        // Open a new template os-window
        let mut cmd = kitty!("@", "launch", "--type", "os-window");
        if let Some(win_name) = &dirspread.win_name {
            cmd.arg("--os-window-title")
                .arg(win_name);
        }
        cmd.output()?;

        // Open a tab for each directory
        if let Some(dirs) = &dirspread.dirs {
            for dir in dirs {
                let dir_name = dir.dir_name.to_owned();
                if let Some(dir_path) = Dirspread::get_full_path(dir_name, &dirspread.parent_dir) {
                    let mut cmd = kitty!("@", "launch", "--type", "tab", "--cwd", dir_path);
                    if let Some(disp_name) = &dir.disp_name {
                        cmd.arg("--tab-title")
                            .arg(disp_name);
                    }
                    cmd.output()?;

                    // Run the startup command specified in the config folder
                    if let Some(on_open) = &dir.on_open {
                        let mut cmd = kitty!("@", "send-text", on_open, "\\r");
                        cmd.output()?;
                    }
                }
            }
        }

        // Close the unused template window
        let mut cmd = kitty!("@", "close-tab", "--match", "index:0");
        cmd.output()?;

        Ok(())
    }
}
