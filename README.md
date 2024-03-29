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
sudo ./metalmetrics --config-file="config.yml" --output-file="output.json"
```



- **Service mode**

```bash
sudo ./metalmetrics --config-file="config.yml" --listen-url="127.0.0.1:9090"
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
    - eth
    - io
    - ip
    - kernel
    - mac
    - network
    - os
    - ram
    - users
    - wake
```



## Output

```
{
    "metrics": [
        {
            "os": "Ubuntu 20.04.4 LTS",
            "mac": "00:01:02:03:04:05",
            "ram": "12 GB (0 GB Used)",
            "network": "RX packets 32301 TX packets 6372",
            "wake": "Supports Wake-on: g\nWake-on: g",
            "users": "name",
            "io": "RD 712836 KB WR 3683508 KB",
            "cpu": "16 CPU (0% Used) 1796.563 MHz",
            "ip": "127.0.0.1",
            "kernel": "Linux 5.4.0-120-generic",
            "eth": "eth0",
            "disk": "269.0 GB (17.0 GB Used)"
        }
    ]
}
```



## Design

![design](design.png)



## License

Project License can be found [here](LICENSE).



## Reference

- [duf](https://github.com/muesli/duf)
- [enabling-wake-on-lan](https://necromuralist.github.io/posts/enabling-wake-on-lan/)
- [health-check-script](https://github.com/SimplyLinuxFAQ/health-check-script)
- [heim](https://github.com/heim-rs/heim)
- [inxi](https://github.com/smxi/inxi)
- [procfs](https://github.com/eminence/procfs)
- [rust-grpc](https://gist.github.com/craftslab/c1b0e5c7f670d6f42a3623d04fddf8c1)
- [rust-psutil](https://github.com/rust-psutil/rust-psutil)
- [sysperf](https://github.com/iandk/sysperf)
- [sysstat](https://github.com/sysstat/sysstat)
- [ubuntu-wake-on-lan](https://sparkydogx.github.io/2019/01/16/ubuntu-wake-on-lan/)
