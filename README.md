# aHashCracker

This is a password hash cracker written in Rust.
It supports cracking password hashes from Linux (/etc/shadow) and Windows (NTLM).

## Features

- Supports MD5, SHA256, SHA512, and NTLM hash types.
- Reads from rockyou wordlist file to perform a dictionary attack.
- Identifies the hash type automatically.
- Docker support for easy deployment and usage.

## Usage

You'll need to [install Rust](https://www.rust-lang.org/tools/install) in order to run the project.

1. Clone the repository:

```bash
git clone https://github.com/onero/ahashcracker.git
cd ahashcracker
```

2. Build the project:

```bash
cargo build --release
```

3. Run the program:

```bash
cargo run
```

## docker-compose

You can also run the program in a Docker container with compose.
For this you just need to have [Docker](https://docs.docker.com/get-docker/) installed.

1. Run the program with the

```bash
docker-compose run cracker
```
