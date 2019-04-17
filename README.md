# FreeVPN Process Spawner - Unix Only

A simple Rust program that starts a local `openvpn` process and fetches the username and password from a *FreeVPN* Server. 

It saves the username/password in `/tmp/` in a file to prevent the openvpn program from asking for username/password.

```text
FreeVPN spawner 0.1
Königsreiter Simon

USAGE:
    freevpn-spawner [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <CONFIG_FILE>                         The OpenVPN config file
    -p, --password-selector <PASSWORD_CSS_SELECTOR>
            The CSS Selector for the password [default: div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(2)]

    -a, --address <FREEVPN_URL>
            The URL to the FreeVPN accounts page [default: https://freevpn.me/accounts]

    -u, --username-selector <USERNAME_CSS_SELECTOR>
            The CSS Selector for the username [default: div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(1)]
```

This is definetely not the most idiomatic way to write rust but I used it just as a simple script as I'm too lazy to always look up the changed passwords of *FreeVPN*.

## Usage

This program requires the programming language to be installed. For more information about the installation of Rust follow [this link](https://rustup.rs/).

To install this program paste this code into a terminal:

```bash
#!/bin/env time bash -ex
git clone https://github.com/koenigsreiter/freevpn-spawner.git
cd freevpn-spawner
cargo install --path . --force

```

And then to execute the program run `sudo freevpn-spawner -c <YOUR_CONFIG_FILE_PATH>`.

## Links

- FreeVPN: https://freevpn.me/
- Rustup: https://rustup.rs/