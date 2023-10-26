use std::{ fmt, fs, process, env, error::Error, path::PathBuf };
use serde::Deserialize;
use serde_json;

mod dirspread_error;

fn main() {
    let terminal = get_terminal();
    let parent_path = get_dirspread_parent();
    let mut ds = Dirspread::new(parent_path, terminal);
    ds.set_from_config().expect("There was a problem with the dsconfig file.");
    ds.get_directories_if_no_config().expect("There was a problem finding dirs within the parent directory");
    ds.open_terminals().expect("There was a problem spreading the directory.");
}

#[derive(Debug)]
enum Terminal {
    TERMINAL,
    KITTY
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terminal::KITTY => write!(f, "Kitty"),
            _ => write!(f, "Terminal")
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ConfigFile {
    pub win_name: Option<String>,
    pub ignored_dirs: Option<Vec<String>>,
    pub dirs: Option<Vec<Directory>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Directory {
    pub disp_name: Option<String>,
    pub dir_name: String,
    pub on_open: Option<String>
}

#[derive(Debug)]
struct Dirspread {
    pub win_name: Option<String>,
    pub parent_dir: PathBuf,
    pub terminal: Terminal,
    pub dirs: Option<Vec<Directory>>,
    pub ignored_dirs: Option<Vec<String>>
}

impl Dirspread {
    pub fn new(parent_dir: PathBuf, terminal: Terminal) -> Dirspread {
        Dirspread { 
            win_name: None,
            parent_dir,
            terminal,
            dirs: None,
            ignored_dirs: None
        }
    }

    fn set_from_config(&mut self) -> Result<(), Box<dyn Error>> {
        let mut config_path = PathBuf::from(&self.parent_dir);
        config_path.push("dsconfig.json");

        println!("{:?}", config_path);

        if config_path.exists() && config_path.is_file() {
            self.save_config(config_path)?;        
        }

        Ok(())
    }

    fn save_config(&mut self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let config = serde_json::from_str::<ConfigFile>(contents.as_str())?;
        
        self.dirs = config.dirs;
        self.ignored_dirs = config.ignored_dirs;
        self.win_name = config.win_name;

        Ok(())
    }

    fn get_directories_if_no_config(&mut self) -> Result<(), Box<dyn Error>> {
        if self.dirs.is_some() {
            return Ok(());
        }

        let contents = fs::read_dir(&self.parent_dir)?;
        let dirs = contents
            .into_iter()
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap().path())
            .filter(|f| f.is_dir())
            .map(|f| String::from(f.file_name().unwrap().to_string_lossy()))
            .filter(|f| {
                if let Some(ignored_dirs) = &self.ignored_dirs {
                    return !ignored_dirs.contains(f);
                }
                true
            })
            .map(|f| {
                Directory { 
                    disp_name: None, 
                    dir_name: f,
                    on_open: None 
                }
            })
            .collect::<Vec<Directory>>();

        self.dirs = Some(dirs);

        Ok(())
    }

    fn open_terminals_macos(&self) -> Result<(), Box<dyn Error>> {
        process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"Terminal\" to activate")
            .output()?;

        if let Some(win_name) = &self.win_name {
            let win_name = win_name.to_owned();
            process::Command::new("osascript")
                .arg("-e")
                .arg(format!("tell application \"Terminal\" to set custom title of the front window to \"{win_name}\""))
                .output()?;
        }

        if let Some(dirs) = &self.dirs {
            for dir in dirs {
                let dir_name = dir.dir_name.to_owned();
                if let Some(dir_path) = Self::get_full_path(dir_name, &self.parent_dir) {
                    process::Command::new("osascript")
                        .arg("-e")
                        .arg("tell application \"System Events\" to tell process \"Terminal\" to keystroke \"t\" using command down")
                        .output()?;
                    process::Command::new("osascript")
                        .arg("-e")
                        .arg(format!("tell application \"Terminal\" to do script \"cd {dir_path}\" in selected tab of the front window"))
                        .output()?;

                    // Set display name if exists
                    if let Some(disp_name) = &dir.disp_name {
                        let disp_name = disp_name.to_owned();
                        process::Command::new("osascript")
                            .arg("-e")
                            .arg(format!("tell application \"Terminal\" to set custom title of the selected tab of the front window to \"{disp_name}\""))
                            .output()?;
                    }


                    // Run the on_open command
                    if let Some(on_open) = &dir.on_open {
                        let on_open = on_open.to_owned();
                        process::Command::new("osascript")
                            .arg("-e")
                            .arg(format!("tell application \"Terminal\" to do script \"{on_open}\" in selected tab of the front window"))
                            .output()?;

                    }
                }
            }
        }
        Ok(())
    }

    fn open_terminals_kitty(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = process::Command::new("kitty");
        cmd.arg("@")
            .arg("launch")
            .arg("--type")
            .arg("os-window");

        if let Some(win_name) = &self.win_name {
            cmd.arg("--os-window-title")
                .arg(win_name);
        }
            
        cmd.output()?;


        if let Some(dirs) = &self.dirs {
            for dir in dirs {
                let dir_name = dir.dir_name.to_owned();
                if let Some(dir_path) = Self::get_full_path(dir_name, &self.parent_dir) {
                    println!("{:?}", dir_path);
                    let mut cmd = process::Command::new("kitty");
                    cmd.arg("@")
                        .arg("launch")
                        .arg("--type")
                        .arg("tab")
                        .arg("--cwd")
                        .arg(dir_path);
                    
                    if let Some(disp_name) = &dir.disp_name {
                        cmd.arg("--tab-title")
                            .arg(disp_name);
                    }

                    cmd.output()?;
                    
                    if let Some(on_open) = &dir.on_open {
                        process::Command::new("kitty")
                            .arg("@")
                            .arg("send-text")
                            .arg(on_open)
                            .arg("\\r")
                            .output()?;
                    }

                }
            }
        }

        process::Command::new("kitty")
            .arg("@")
            .arg("close-tab")
            .arg("--match")
            .arg("index:0")
            .output()?;

        Ok(())
    }


    fn open_terminals(&self) -> Result<(), Box<dyn Error>> {
        match &self.terminal {
            Terminal::KITTY => &self.open_terminals_kitty(),
            Terminal::TERMINAL => &self.open_terminals_macos()
        };

        Ok(())
    
    }

    fn get_full_path(dir_name: String, parent: &PathBuf) -> Option<String> {
        let mut path = PathBuf::from(parent);
        path.push(dir_name);

        if !path.is_dir() {
            return None
        }

        Some(String::from(path.to_string_lossy()))
    }

}

fn get_terminal() -> Terminal {
    if let Some(_) = env::var_os("KITTY_WINDOW_ID") {
        return Terminal::KITTY;
    }

    Terminal::TERMINAL
}


fn get_dirspread_parent() -> PathBuf {
    let parent_dir = env::args().nth(1);
    let parent_dir = if parent_dir.is_some() {
        let dir_path = PathBuf::from(parent_dir.unwrap());
        if dir_path.exists() && dir_path.is_dir() {
            fs::canonicalize(dir_path).expect("This directory cannot be spread.")
        } else {
            panic!("The given directory could not be found");
        }
    } else {
        let mut curr_dir = env::current_dir().expect("This directory cannot be spread.");
        curr_dir.pop();
        fs::canonicalize(curr_dir).expect("This directory cannot be spread.")
    };

    parent_dir
}

