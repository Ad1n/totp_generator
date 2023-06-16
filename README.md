# TOTP Generator

This is a command-line tool written in Rust that generates Time-Based One-Time Password (TOTP) codes using a secret key and password provided as command-line arguments. The TOTP code is then copied to the clipboard for easy access.

## Prerequisites

Before using this tool, ensure that you have Rust and Cargo installed on your system. You can install them by following the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Usage

To use the TOTP generator, follow these steps:

1. Clone or download this repository to your local machine.
2. Open a terminal and navigate to the project's directory.
3. Build the project by running the following command:
   ```
   cargo build --release
   ```
4. Run the TOTP generator with the secret key and password as command-line arguments:
   ```
   ./target/release/totp-generator <secret_key> <password>
   ```
   Replace `<secret_key>` with the TOTP secret key and `<password>` with the desired password.

## Example

Here's an example command to generate a TOTP code and copy it to the clipboard:

```
./target/release/totp-generator 1L25QEA0KRV2HIU1G0A120D1KWIL5W32 mypassword123
```

## Security Considerations

Please note that the TOTP secret key and password provided as command-line arguments may be visible in the command history or system logs. Exercise caution when using this tool on shared systems.

## Troubleshooting

If you encounter any issues or errors while using the TOTP generator, feel free to open an issue in the GitHub repository: [totp_generator](https://github.com/Ad1n/totp_generator)

## Contributions

Contributions to this project are welcome! If you have any improvements or suggestions, feel free to open a pull request or submit an issue.

## License

This project is licensed under the [MIT License](LICENSE).
