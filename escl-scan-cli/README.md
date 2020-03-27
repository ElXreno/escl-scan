# escl-scan-cli

[![Copr build status](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan/package/escl-scan/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan)
[![github actions](https://github.com/ElXreno/escl-scan/workflows/Rust/badge.svg)](https://github.com/ElXreno/escl-scan/actions)
[![codecov](https://codecov.io/gh/ElXreno/escl-scan/branch/master/graph/badge.svg)](https://codecov.io/gh/ElXreno/escl-scan)
[![dependency status](https://deps.rs/repo/github/elxreno/escl-scan/status.svg)](https://deps.rs/repo/github/elxreno/escl-scan)

---

```
escl-scan-cli 0.1.1
ElXreno <elxreno@gmail.com>
CLI for escl-scan

USAGE:
    escl-scan-cli [FLAGS] [OPTIONS] <ip> <output file>

FLAGS:
    -f, --force      Force scan and override output file
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dpi <dpi>    Scan resolution [default: 75]

ARGS:
    <ip>             IP of scanner
    <output file>    Destination file
```

# Install
### As package (recommend):
Fedora [Copr](https://copr.fedorainfracloud.org/coprs/elxreno/escl-scan): `sudo dnf copr enable elxreno/escl-scan -y && sudo dnf install escl-scan`

### As binary:
```
cargo install escl-scan-cli
# by default installed at ~/.cargo/bin, you may add it to path:
export PATH=$PATH:~/.cargo/bin
escl-scan-cli
```
