# git-find

[![License: CC0-1.0](https://img.shields.io/badge/License-CC0%201.0-lightgrey.svg)](http://creativecommons.org/publicdomain/zero/1.0/)
[![Build Status](https://travis-ci.org/davidB/git-find.svg?branch=master)](https://travis-ci.org/davidB/git-find)
[![Crates.io](https://img.shields.io/crates/v/git-find.svg)](https://crates.io/crates/git-find)
[![Documentation](https://docs.rs/git-find/badge.svg)](https://docs.rs/git-find/)

A tool (cli & lib) to find local git repositories.

## Why

Because I need a tool to list and to reorg my local git repositories.

## Usage Cli

```sh
cd my_projects
git-find
```

## Install

## Related and similar

Some tools to help management of multi repository
But not the same features, else no need to re-do.

### Informations

* [peap/git-global: Keep track of all your git repositories.](https://github.com/peap/git-global) (I quickly look at the source to estimate my contribution to add features, but the "potentials" changes are too many and could change the goal usage of the tool)
* [totten/git-scan: CLI tool for scanning/updating git repos](https://github.com/totten/git-scan/)
* [fboender/multi-git-status: Show uncommitted, untracked and unpushed changes for multiple Git repos](https://github.com/fboender/multi-git-status)

### Actions (broadcast)

* [gr - A tool for managing multiple git repositories](http://mixu.net/gr/)
* [mu-repo](http://fabioz.github.io/mu-repo/), Tool to help working with multiple git repositories (short for Multiple Repositories).
* [mr](http://myrepos.branchable.com/) which is a tool to manage all your version control repositories.
* [Repo command reference  |  Android Open Source Project](https://source.android.com/setup/develop/repo)

## TODO

* find a rust template engine that support calling method (getter) on current field (or contribute to gtmpl as it's a feature of go template)
* internally use stream / queue instead of Vector
* optimize