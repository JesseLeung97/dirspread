use std::{ 
    fs, 
    env, 
    error::Error, 
    path::PathBuf 
};
use serde::Deserialize;
use serde_json;

mod dirspread_error;
mod terminals;
use terminals::{ macos::Macos, kitty::Kitty, terminal_interface::TerminalInterface };

fn main() {
    let terminal = get_terminal();
    if let Some(parent_path) = get_dirspread_parent() {
        let mut ds = Dirspread::new(parent_path, terminal);
        ds.set_from_config().expect("There was a problem with the dsconfig file.");
        ds.get_directories_if_no_config().expect("There was a problem finding dirs within the parent directory");
        ds.open_terminals().expect("There was a problem spreading the directory.");
    } else {
        println!("There was a problem spreading the given directory.");
    }
}

#[derive(Debug)]
pub enum Terminal {
    TERMINAL,
    KITTY
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
pub struct Directory {
    pub disp_name: Option<String>,
    pub dir_name: String,
    pub on_open: Option<String>
}

#[derive(Debug)]
pub struct Dirspread {
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

    // If there is no config, find all the directories within the parent and create a default entry
    // for them
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

    fn open_terminals(&self) -> Result<(), Box<dyn Error>> {
        match &self.terminal {
            Terminal::KITTY => Kitty::open_terminals(&self)?,
            Terminal::TERMINAL => Macos::open_terminals(&self)?
        };

        Ok(())
    }

    // Translate relative paths to absolute paths
    fn get_full_path(dir_name: String, parent: &PathBuf) -> Option<String> {
        let mut path = PathBuf::from(parent);
        path.push(dir_name);

        if !path.is_dir() {
            return None
        }

        Some(String::from(path.to_string_lossy()))
    }
}

// Check which terminal is calling the script
fn get_terminal() -> Terminal {
    if let Some(_) = env::var_os("KITTY_WINDOW_ID") {
        return Terminal::KITTY;
    }

    Terminal::TERMINAL
}

// Get path of the directory where the script was called
fn get_dirspread_parent() -> Option<PathBuf> {
    let parent_dir = env::args().nth(1);
    let parent_dir = if parent_dir.is_some() {
        let dir_path = PathBuf::from(parent_dir.unwrap());
        if dir_path.exists() && dir_path.is_dir() {
            fs::canonicalize(dir_path).expect("This directory cannot be spread.")
        } else {
            return None;
        }
    } else {
        let curr_dir = env::current_dir().expect("This directory cannot be spread.");
        println!("{:?}", curr_dir);
        fs::canonicalize(curr_dir).expect("This directory cannot be spread.")
    };

    Some(parent_dir)
}

