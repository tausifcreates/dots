#
# ~/.bash_profile
#
[[   ~/.bashrc ]] && . ~/.bashrc
. "$HOME/.cargo/env"

export PATH=$PATH:~/.local/bin

# Run river if login from tty1
if [ -z $DISPLAY ] && [ "$(tty)" = "/dev/tty1" ]; then
	river
fi

if [ -n "$DESKTOP_SESSION" ];then
    eval $(gnome-keyring-daemon --start)
    export SSH_AUTH_SOCK
fi
