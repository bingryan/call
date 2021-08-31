# tutorial

## config

```shell
> pwd
/Users/user/.call
> ls
config.toml   template.toml
```

`config.toml` is the basic configuration file of `call`
```shell
template = "template.toml"
call_config_path = ".local"
```

`call_config_path` is the selected path of Call.yml, the default is the current(.).
If you want to put it in the .local folder of the current project, then it should be set like this

```shell
call_config_path = ".local"
```

## template

When you run `call i`. You want to use your usual template, then you can set it like this：

```shell
> pwd
/Users/user/.call
> cat template.toml
call:
  config:
    active:
      openssh:
        - dev
    runner: make
  mapping:
      src: .
      dest: ~/workspace/call
      exclude:
          - ./target
          - README.md
  server:
        openssh:
          dev:
              host:
                - 127.0.0.1
              port: 22
              authentication_type: openssh
              username: rust
```

