# Dirspread
A cli tool to quickly open a directory of directories in terminal and terminal emulators.


## Installation

### Manual
1. Download the latest binary.
2. Move the binary into your `/usr/local/bin` folder and ensure the file is executable


## Usage
The binary is named `dirspread`.  Running `dirspread` with no arguments will attempt to open a terminal tab for all directories in the current directory.  Otherwise, a target directory can be specified with a cli argument using the `dirspread <target folder>` format.  Both relative and absolute paths will resolve correctly.

```
// Open the current directory
dirspread

// Open the parent of the current directory
dirspread ../

// Open using an absolute path
dirspread /Users/myuser/myprojects/myprojectfolder
```

## Configuration
If no configuration file is present, dirspread will attempt to open a terminal tab for all directories included in the current directory.  This is not usually the desired behavior and can be remedied by including a `dsconfig.json` file in the directory you are attempting to dirspread.  

`winName?: String` The title (top center) of the new terminal window (Optional)
`ignoredDirs?: String[]` A list of directory names dirspread should ignore (Optional)
`dirs?: Directory[]` A list of directories which will be opened in a new tab (Optional)
```
Directory {
    dispName?: String,  // The tab title (Optional)
    dirName: String,    // The directory name to be opened
    onOpen?: String     // A command to be run on startup (Optional)
}
```

The following is a sample configuration file.


```
{
    "winName": "Example Window",
    "ignoredDirs": [
        ".git",
        ".debug"
    ],
    "dirs": [
        {
            "dispName": "Nvim",
            "dirName": "rust-app",
            "onOpen": "nvim ."
        },
        {
            "dispName": "Docker",
            "dirName": "rust-app",
            "onOpen": "cd docker && docker-compose up -d"
        },
        {
            "dirName": "react-app",
        }

    ]
}
```
