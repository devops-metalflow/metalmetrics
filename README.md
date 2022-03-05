# metalmetrics

[![Actions Status](https://github.com/devops-metalflow/metalmetrics/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/devops-metalflow/metalmetrics/actions?query=workflow%3ACI)
[![License](https://img.shields.io/github/license/devops-metalflow/metalmetrics.svg?color=brightgreen)](https://github.com/devops-metalflow/metalmetrics/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/devops-metalflow/metalmetrics.svg?color=brightgreen)](https://github.com/devops-metalflow/metalmetrics/tags)
[![Gitter chat](https://badges.gitter.im/craftslab/devops-metalflow.png)](https://gitter.im/craftslab/devops-metalflow)



## Introduction

*metalmetrics* is a worker of *[metalflow](https://github.com/devops-metalflow/metalflow/)* written in Rust.



## Prerequisites

- Rust >= 1.59.0



## Run

- **Local mode**

```bash
./metalmetrics --config-file="config.yml" --output-file="output.json"
```



- **Service mode**

```bash
./metalmetrics --config-file="config.yml" --listen-url="127.0.0.1:9090"
```



## Usage

```
USAGE:
    metalmetrics [OPTIONS] --config-file <NAME>

OPTIONS:
    -c, --config-file <NAME>    Config file (.yml)
    -h, --help                  Print help information
    -l, --listen-url <URL>      Listen url (host:port)
    -o, --output-file <NAME>    Output file (.json|.txt)
    -V, --version               Print version information
```



## Settings

*metalmetrics* parameters can be set in the directory [config](https://github.com/devops-metalflow/metalmetrics/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/devops-metalflow/metalmetrics/blob/main/src/config/config.yml):

```yaml
apiVersion: v1
kind: worker
metadata:
  name: metalmetrics
spec:
  metrics:
    - cpu
    - disk
    - io
    - ip
    - kernel
    - mac
    - network
    - os
    - ram
    - users
```



## Design

![design](design.png)



## License

Project License can be found [here](LICENSE).



## Reference

- [health-check-script](https://github.com/SimplyLinuxFAQ/health-check-script)
- [heim](https://github.com/heim-rs/heim)
- [inxi](https://github.com/smxi/inxi)
- [rust-grpc](https://gist.github.com/craftslab/c1b0e5c7f670d6f42a3623d04fddf8c1)
- [rust-psutil](https://github.com/rust-psutil/rust-psutil)
- [sysperf](https://github.com/iandk/sysperf)
- [sysstat](https://github.com/sysstat/sysstat)
