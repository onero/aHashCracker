# aHashCracker

This is a password hash cracker written in Rust.
It supports cracking password hashes from Linux (/etc/shadow) and Windows (NTLM).

## Features

- Supports MD5, SHA256, SHA512, and NTLM hash types.
- Reads from rockyou wordlist file to perform a dictionary attack.
- Identifies the hash type automatically.
- Docker support for easy deployment and usage.

## Usage

### Running with docker-compose

NB. The container ships with rockyou.txt included!

You can run the program in a Docker container with compose.
For this you just need to have [Docker](https://docs.docker.com/get-docker/) installed.

1. Download the [docker-compose.yml](https://github.com/onero/aHashCracker/blob/master/docker-compose.yml)
2. Run the program with docker-compose

```bash
docker-compose run cracker
```

### Running from the binary

1. Grab the latest [release](https://github.com/onero/aHashCracker/releases)
2. Download a wordlist (rockyou was used, but feel free to use any and rename to "rockyou.txt") and place it next to the program in wordlist/rockyou.txt
3. Execute binary and pwn passwords!

### Example of program running

The program will prompt for the hash line

```bash
Enter the line from /etc/shadow or SAM DB:
adamino:502:c46b9e588fa0d112de6f59fd6d58eae3:C196CAE1F0F364BAF748EE0E7F753A15:::

...
[9997] No match...
[9998] No match...
[9999] No match...
Cracked password: Attempt: 10000
Result: adamino:kupal
```

### Development

You'll need to [install Rust](https://www.rust-lang.org/tools/install) in order to run the project.

1. Clone the repository:

```bash
git clone https://github.com/onero/aHashCracker.git
cd aHashCracker
```

2. Download a wordlist (rockyou was used, but feel free to use any and rename to "rockyou.txt") and place it next to the program in wordlist/rockyou.txt

3. Build the project:

```bash
cargo build --release
```

4. Run the program:

```bash
cargo run
```

## Contributing

Contributions to this project are welcome. Please create a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
