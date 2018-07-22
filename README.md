# git-find

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0%201.0-lightgrey.svg)](http://creativecommons.org/publicdomain/zero/1.0/)
[![Build Status](https://travis-ci.org/davidB/git-find.svg?branch=master)](https://travis-ci.org/davidB/git-find)
[![Build status](https://ci.appveyor.com/api/projects/status/03ok7d1x3h0i5kay?svg=true)](https://ci.appveyor.com/project/davidB/git-find)
[![Crates.io](https://img.shields.io/crates/v/git-find.svg)](https://crates.io/crates/git-find)
[![Documentation](https://docs.rs/git-find/badge.svg)](https://docs.rs/git-find/)

A tool (cli & lib) to find local git repositories.

<!-- vscode-markdown-toc -->
* [Why](#Why)
* [Usage Cli](#UsageCli)
	* [Template format](#Templateformat)
		* [Possibles values](#Possiblesvalues)
		* [For scripting](#Forscripting)
		* [Samples](#Samples)
* [Install](#Install)
	* [From cargo](#Fromcargo)
	* [From binaries](#Frombinaries)
	* [From source](#Fromsource)
* [Related and similar](#Relatedandsimilar)
	* [Informations](#Informations)
	* [Actions (broadcast)](#Actionsbroadcast)
* [TODO](#TODO)

<!-- vscode-markdown-toc-config
	numbering=false
	autoSave=true
	/vscode-markdown-toc-config -->
<!-- /vscode-markdown-toc -->

## <a name='Why'></a>Why

Because I need a tool to list and to reorg my local git repositories.

## <a name='UsageCli'></a>Usage Cli

```sh
$> git find -h
git-find 0.3.2
davidB
A tool (cli & lib) to find local git repositories.

USAGE:
    git-find [FLAGS] [OPTIONS] [DIR]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.) print on stderr

OPTIONS:
    -t, --tmpl <format>    format of the output print on stdout [default: {{ .path.file_name }} {{ .path.full }}
                           {{with .remotes.origin}} {{ .name }} {{.url_full}} {{end}}]

ARGS:
    <DIR>    root directory of the search [default: .]
```

* broadcast `git status` to every repositories

```sh
git find -t 'cd {{ .path.full }}; echo "\n\n----\n$PWD"; git status' | sh
````

### <a name='Templateformat'></a>Template format

The template format is a subset of [golang text/template](https://golang.org/pkg/text/template/).

#### <a name='Possiblesvalues'></a>Possibles values

*!! Experimental: values could change with future release !!*

* .path
  * .file_name
  * .full
* .remotes
  * .<name_of_remote> : eg 'origin'
    * .name
    * .url_full
    * .url_host
    * .url_path
* .working_paths}}
  * .conflicted : list of path as string
  * .modified : list of path as string
  * .added : list of path as string
  * .deleted : list of path as string
  * .renamed : list of path as string
  * .untracked : list of path as string

#### <a name='Forscripting'></a>For scripting

1. Use explicit template within your script (the default template could change with each release)
1. If the template is a shell script (the tips could be used with every interpreter: python, ruby, ...), then you can
```sh
# run it directly
git find -t '...' | sh

# generate a script and run it later (maybe after review)
git find -t '...' > myscript.sh
sh myscript.sh

```


#### <a name='Samples'></a>Samples

* to list local repository

```tmpl
{{ .path.file_name }}\t{{ .path.full }}
```

* to list local repository with origin url

```tmpl
{{ .path.file_name }}\t{{ .path.full }}\t{{with .remotes.origin}} {{ .name }} {{.url_full}} {{.url_host}} {{.url_path}} {{end}}
````

* to create a sh script to "git fetch" on every repository

```tmpl
cd {{ .path.full }}; echo "\n\n---------------------------------------------\n$PWD"; git fetch
```

* to create a sh script to move git repository under $HOME/src (same layout as go workspace)

```tmpl
echo "\n\n---------------------------------------------\n"
PRJ_SRC="{{ .path.full }}"
{{if .remotes.origin}}
PRJ_DST="$HOME/src/{{ .remotes.origin.url_host }}/{{ .remotes.origin.url_path}}"
{{else}}
PRJ_DST=$HOME/src/_local_/{{ .path.file_name}}
{{end}}
if [ ! -d "$PRJ_DST" ] ; then
  read -p "move $PRJ_SRC to $PRJ_DST ?" answer
  case $answer in
    [yY]* )
        mkdir -p $(dirname "$PRJ_DST")
        mv "$PRJ_SRC" "$PRJ_DST"
        ;;
    * ) ;;
  esac
fi
```
* to list repo with some info (the default template of version 0.4.1)

```tmpl
{{with .working_paths}}{{if .conflicted}}C{{else}} {{end}}{{if .modified}}M{{else}}{{if .added}}M{{else}}{{if .deleted}}M{{else}}{{if .renamed}}M{{else}} {{end}}{{end}}{{end}}{{end}}{{if .untracked}}U{{else}} {{end}}{{end}}\t{{ .path.file_name }}\t{{ .path.full }}\t{{with .remotes.origin}} {{ .name }} {{.url_full}} {{end}}
```

## <a name='Install'></a>Install

### <a name='Fromcargo'></a>From cargo

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install via:

```sh
cargo install git-find
```

Note that rust version *1.26.0* or later is required.

### <a name='Frombinaries'></a>From binaries

*!! Experimental !!*

The [release page](https://github.com/davidb/git-find/releases) includes precompiled binaries for Linux, macOS and Windows.

* download archives for your OS
* unarchive it, and move the executable into the PATH

```sh
tar -xzvf git-find_0.3.2-x86_64-unknown-linux-gnu.tar.gz
chmod a+x git-find
mv git-find $HOME/bin
rm git-find_0.2.2-linux.tar.gz
```

### <a name='Fromsource'></a>From source

```sh
git clone https://github.com/davidB/git-find

# Build
cd git-find
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install
```

## <a name='Relatedandsimilar'></a>Related and similar

Some tools to help management of multi repository
But not the same features, else no need to re-do.

### <a name='Informations'></a>Informations

* [peap/git-global: Keep track of all your git repositories.](https://github.com/peap/git-global) (I quickly look at the source to estimate my contribution to add features, but the "potentials" changes are too many and could change the goal usage of the tool)
* [totten/git-scan: CLI tool for scanning/updating git repos](https://github.com/totten/git-scan/)
* [fboender/multi-git-status: Show uncommitted, untracked and unpushed changes for multiple Git repos](https://github.com/fboender/multi-git-status)

### <a name='Actionsbroadcast'></a>Actions (broadcast)

* [gr - A tool for managing multiple git repositories](http://mixu.net/gr/)
* [mu-repo](http://fabioz.github.io/mu-repo/), Tool to help working with multiple git repositories (short for Multiple Repositories).
* [mr](http://myrepos.branchable.com/) which is a tool to manage all your version control repositories.
* [Repo command reference  |  Android Open Source Project](https://source.android.com/setup/develop/repo)

## <a name='TODO'></a>TODO

* find a rust template engine that support calling method (getter) on current field (or contribute to gtmpl as it's a feature of go template)
* internally use stream / queue instead of Vector
* build linux binary with musl (see https://github.com/emk/rust-musl-builder)
* optimize
