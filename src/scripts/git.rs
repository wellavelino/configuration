//setup git

//cp scripts/git/.gitconfig  ~/.gitconfig
//source ~/.gitconfig
//cp  scripts/git/.gitignore_global ~/.gitignore_global
//source ~/.gitignore_global

use std::fs;

fn move_git_config() -> std::io::Result<()> {
    fs::copy(
        "../scripts/git/.gitconfig",
        "get whoami from executing a shell",
    )?;
    //runs a source
    Ok(()); //check for errors in pattern matching.
}

fn move_git_ignore() -> sd::io::Result<()> {
    fs::copy(
        "../scripts/git/.gitconfig",
        "get whoami from executing a shell",
    )?;
    //runs a source
    Ok(()); //check for errors in pattern matching.
}
