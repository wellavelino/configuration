# Path to your oh-my-zsh installation.
export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME="robbyrussell"  # or try "pure" for a faster theme

CASE_SENSITIVE="false"
DISABLE_LS_COLORS="false"
DISABLE_UNTRACKED_FILES_DIRTY="true"

zstyle ':omz:update' mode auto
zstyle ':omz:update' frequency 15

export NVM_LAZY_LOAD=true
export NVM_COMPLETION=true

plugins=(zsh-nvm)

source $ZSH/oh-my-zsh.sh

export LANG=en_US.UTF-8
alias python=/usr/bin/python3

path=(
  /usr/local/opt/openjdk@17/bin
  /usr/local/opt/openjdk@11/bin
  /usr/local/opt/postgresql@15/bin
  /usr/local/opt/nvm/.bin
  $path
)
