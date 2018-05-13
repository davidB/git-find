# git-find

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0%201.0-lightgrey.svg)](http://creativecommons.org/publicdomain/zero/1.0/)
[![Build Status](https://travis-ci.org/davidB/git-find.svg?branch=master)](https://travis-ci.org/davidB/git-find)
[![Crates.io](https://img.shields.io/crates/v/git-find.svg)](https://crates.io/crates/git-find)
[![Documentation](https://docs.rs/git-find/badge.svg)](https://docs.rs/git-find/)

A tool (cli & lib) to find local git repositories.

<!-- vscode-markdown-toc -->
* [Why](#Why)
* [Usage Cli](#UsageCli)
* [Install](#Install)
	* [Via rust toolchain](#Viarusttoolchain)
	* [Via pre-build binaries](#Viapre-buildbinaries)
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
$> git-find -h

git-find 0.2.0
davidB
A tool (cli & lib) to find local git repositories.

USAGE:
    git-find [FLAGS] [OPTIONS] [DIR]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.) print on stderr

OPTIONS:
    -t, --tmpl <format>    format of the output print on stdout [default: {{ .path.file_name }}    {{ .path.full }}]

ARGS:
    <DIR>    root directory of the search [default: .]
```

* broadcast `git status` to every repositories

```sh
git find -t 'cd {{ .path.full }}; echo "\n\n---------------------------------------------\n$PWD"; git status' | sh
````

### Template format

The template format is a subset of [golang text/template](https://golang.org/pkg/text/template/).

### Possibles values

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

#### Samples

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
PRJ_SRC={{ .path.full }}
{{with .remotes.origin}}
PRJ_DST=$HOME/src/{{ .url_host }}/{{ .url_path}}
if [ ! -d $PRJ_DST ] ; then
  read -p "move $PRJ_SRC to $PRJ_DST ?" answer
  case $answer in
    [yY]* )
        mkdir -p $(dirname $PRJ_DST)
        mv $PRJ_SRC $PRJ_DST
        ;;
    * ) ;;
  esac
fi
{{end}}
```

## <a name='Install'></a>Install

### <a name='Viarusttoolchain'></a>Via rust toolchain

```sh
cargo install git-find
```

### <a name='Viapre-buildbinaries'></a>Via pre-build binaries

*!! Experimental !!*

* download archives for your OS from https://github.com/davidB/git-find/releases
* unarchive it, and move the executable into the PATH

```sh
tar -xzvf git-find_0.2.2-linux.tar.gz
chmod a+x git-find
mv git-find $HOME/bin
rm git-find_0.2.2-linux.tar.gz
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