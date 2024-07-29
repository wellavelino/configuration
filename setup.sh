#!/usr/bin/env bash

 #To execute: save and `chmod +x ./setup.sh` 
install_homebrew() {
    echo "Installing Homebrew"
    if command -v brew >/dev/null 2>&1; then
        echo "Homebrew is already installed."
        return 0
    fi
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # Verify installation
    if command -v brew >/dev/null 2>&1; then
        echo "Homebrew has been successfully installed."
        return 0
    else
        echo "Homebrew installation failed."
        return 1
    fi
}

install_brew_bundle() {
  echo "Installing Brew Bundle"
   brew bundle --file="./homebrew/Brewfile"
}

## Last Step after installing all dependencies
##stow .
