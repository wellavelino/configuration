# dot-filer

Rust project that manages, installs and updates your dotfiles

Porting of https://github.com/wellavelino/ansible_qa_playbook/

instead of using ansible will use Rust.

// Roadmap/ TODO
Define what you want to installs
Pass a .dotfiles repo or install via the CLI
Sync remote with local dotfiles.
Add macos settings (dark mode, fonts etc..)
Port all shell scripts to Rust
Make sure HomeBrew can respect OS and processor arch. x86 or ARM (the installation location changes)
Install Nvim from source
Add Docker setup
Install Notion or obsidian (decide wich one)
Install wezterm via CLI
Install NerdFonts
Install Terminal completion (see nvim repo)
