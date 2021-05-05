<p align="center">
  <img src="docs/logo.png" width="50%" syt height="50%" />
</p>

# Call: Make remote development more elegant

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


