# metalmetrics-rs

[![Actions Status](https://github.com/craftslab/metalmetrics-rs/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/craftslab/metalmetrics-rs/actions?query=workflow%3ACI)
[![License](https://img.shields.io/github/license/craftslab/metalmetrics-rs.svg?color=brightgreen)](https://github.com/craftslab/metalmetrics-rs/blob/master/LICENSE)
[![Tag](https://img.shields.io/github/tag/craftslab/metalmetrics-rs.svg?color=brightgreen)](https://github.com/craftslab/metalmetrics-rs/tags)



## Introduction

*metalmetrics-rs* is a worker of *[metalflow](https://github.com/craftslab/metalflow/)* written in Rust.



## Prerequisites

- Rust >= 1.52.0



## Run

- **Local mode**

```bash
./metalmetrics-rs --config-file="config.yml" --inxi-file="inxi" --output-file="output.json"
```



- **Service mode**

```bash
./metalmetrics-rs --config-file="config.yml" --inxi-file="inxi" --listen-url="127.0.0.1:9090"
```



## Usage

```
USAGE:
    metalmetrics-rs [OPTIONS] --config-file <NAME>

OPTIONS:
    -c, --config-file <NAME>    config file (.yml)
    -h, --help                  Print help information
    -i, --inxi-file <NAME>      inxi file (/path/to/inxi)
    -l, --listen-url <URL>      listen url (host:port)
    -o, --output-file <NAME>    output file (.json|.txt|.xlsx)
    -V, --version               Print version information
```



## Settings

*metalmetrics-rs* parameters can be set in the directory [config](https://github.com/craftslab/metalmetrics-rs/blob/master/src/config).

An example of configuration in [config.yml](https://github.com/craftslab/metalmetrics-rs/blob/master/src/config/config.yml):

```yaml
apiVersion: v1
kind: worker
metadata:
  name: metalmetrics-rs
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
    - system
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
