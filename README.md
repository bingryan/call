<p align="center">
  <img src="docs/logo.png" width="50%" syt height="50%" />
</p>

# Call: Make remote development more elegant

[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)

[![Build Status](https://travis-ci.com/bingryan/call.svg?branch=main)](https://travis-ci.com/bingryan/call)

-------

## What does it do

`Call` is an easy-to-use command tools for remote development. It helps you to build remote development easily and elegant.
It can work with [makefile](https://github.com/mirror/make) and [justfile](https://github.com/casey/just).

`Call` provides three ways.

* **openssh**
  SSH Login Without Password
  
* **password**
  SSH Login With Password
  
* **keypair**
  SSH Login With Private Key File


## Quick Start

It is super easy to get started with your first project.

### Step 1: install call command tools

```shell
cargo install --git  https://github.com/bingryan/call.git
```
or

```shell
cargo install rust-call
```

### Step 2: init call 

```shell
cd your_project_homepage
call i
```

there will be a `call.yml` file at `your_project_homepage`. Then you can configure `call.yml`。such as:
```yaml
call:
  config:
    active:
      openssh:
        - dev  # active server label
    runner: make  # make -> makefile , just -> justfile
  mapping:
      src: . # current dir
      dest: ~/workspace/call  # remote path
      exclude: # Ignored directories in .gitignore will also be ignored
          - ./target
          - README.md
  server:
        openssh:
          dev: # label for openssh server
              host:
                - 192.168.2.49 # multiple
              port: 22
              authentication_type: openssh
              username: rust # remote server username


```

### Step 3: run call

replace `make xxx` with `call xxx` command, when your project has makefile.

replace `just xxx` with `call xxx` command, when your project has justfile.


## Requirements

- **openssh** (rsync)
- **password** (rsync,sshpass)
- **keypair** (rsync)

```shell
# Debian/Ubuntu
$ sudo apt-get install rsync
$ sudo apt-get install sshpass

# Red Hat
$ sudo yum install rsync
$ sudo yum install sshpass

# Arch Linux
$ sudo pacman -S rsync
$ sudo pacman -S sshpass

# MacOS
$ brew install rsync
$ brew install http://git.io/sshpass.rb(if notwork, copy raw content to `sshpass.rb`: brew install sshpass.rb)
```

## Architecture

<p align="center">
  <img src="docs/architecture.png" width="50%" syt height="50%" />
</p>

## Contributing

Contributors are welcomed to join this project. Please check [CONTRIBUTING](./CONTRIBUTING.md) about how to contribute to this project.
