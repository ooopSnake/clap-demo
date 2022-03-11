# clap derive demo

## build

```bash
cargo build --bin fake-docker
```

## run example

```bash
fake-docker --help

# subcommand log example
fake-docker log -f --since "1h" -c 1fab3

# subcommand ps example
fake-docker ps fake-container-id
```

### help info

```text
clap-demo 0.1.0
Example show how to use this demo Subcommand: - log ```bash ./fake-docker log -f --since "1h" -c
1fab3 ``` - ps ```bash ./fake-docker ps 1fab3 ```

USAGE:
    fake-docker <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    log     show container logs
    ps      show container status

```