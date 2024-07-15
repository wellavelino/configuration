#[derive(Debug, Clone, Copy)]
pub enum BrewPackageType {
    Cask,
    Formula,
}

pub const BREW_CASK_PACKAGES: &[&str] = &[
    "google-chrome",
    "caffeine",
    "adoptopenjdk8",
    "android-studio",
    "android-sdk",
    "iterm2",
    "emacs",
    "postman",
    "spotify",
    "proxyman",
];

pub const BREW_PACKAGES: &[&str] = &[
    "git",
    "pyenv",
    "python3",
    "rbenv",
    "ruby-build",
    "git-extras",
    "nano",
    "bash-git-prompt",
    "gradle",
    "pyenv",
    "jq",
    "htop",
    "ncdu",
    "diff-so-fancy",
];

pub struct BrewPackage<'a> {
    pub name: &'a str,
    pub package_type: BrewPackageType,
}

pub const ALL_BREW_PACKAGES: &[BrewPackage] = &[
    // Cask packages
    BrewPackage { name: "google-chrome", package_type: BrewPackageType::Cask },
    BrewPackage { name: "caffeine", package_type: BrewPackageType::Cask },
    // ... add all cask packages here

    // Formula packages
    BrewPackage { name: "git", package_type: BrewPackageType::Formula },
    BrewPackage { name: "pyenv", package_type: BrewPackageType::Formula },
    // ... add all formula packages here
];
