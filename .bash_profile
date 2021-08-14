#
# ~/.bash_profile
#
[[   ~/.bashrc ]] && . ~/.bashrc
. "$HOME/.cargo/env"

export PATH=$PATH:~/.local/bin

if [ -z $DISPLAY ] && [ "$(tty)" = "/dev/tty1" ]; then
  river
fi