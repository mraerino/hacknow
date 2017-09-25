# HackNow

Utility for automation of project directories and switching workspaces based on the GitHub directory structure - `User/Project`

This is a RIIR for fun of [mraerino/hacknow](https://github.com/mraerino/hacknow).

## Functionality

- Directory structure: `<projects-dir>/<user>/<repo>`
- Checks if a directory for the specified repo exists
- No: Clones github repo into new directory
- Yes: Does `git fetch --all` for this repo
- Then changes into the project directory

## Getting started

This is available on cargo:

`cargo install hacknow``

### Shell function

If you want your shell to switch into the project directory, you need to create a small function inside your shell config (`.bashrc`, `.zshrc` or similar)

```sh
hn() {
  PROJECTPATH=$(hacknow "$@")
  RETVAL=$?
  cd $PROJECTPATH
  return $RETVAL
}
```

Choose a different alias if `hn` doesn't work for you.

*Please note: On failure hacknow will echo just `.` to stdout, so it should be safe to call `cd` on it in every situation.*

## Usage

```sh
hacknow <user>/<repo> # Plain (without dir change)
hn <user>/<repo> # Needs a shell function (see above)
cd $(hacknow <user>/<repo>) # Alternative if the shell function does not work for you
```

### Options

```
-d|--dir <dir>    Choose base directory for your projects (default: $HOME)
--ssh             Use ssh protocol for git remote
```

### Examples

```sh
$ hn mraerino/gitmoji
$ hn mraerino/hacknow -d ~/src
$ hn festify/app --ssh
```

## Ideas / Roadmap

- Bash completion
- Execute scripts inside project directory (like `nvm`, `virtualenv`, `yarn install`, IDE launch...)