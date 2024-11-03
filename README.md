# File Encryption/Decryption Application

This Rust application provides secure AES-256 encryption and decryption of files. It allows you to encrypt a file into an encrypted format or decrypt an encrypted file back to its original form, using a password for security. The application uses AES-256 in CBC mode and supports password-based encryption with a custom key and initialization vector.

## Features

- **AES-256 CBC Encryption**: Encrypts files using AES-256 in CBC mode.
- **Password-based Encryption**: Requires a password for both encryption and decryption.
- **Cross-Platform Support**: Runs on Linux, macOS, and Windows.
- **Command-line Interface**: Simple and flexible CLI for encrypting and decrypting files.

## Prerequisites

- **Rust**: Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/rmalcomber/file-encryptor.git
   cd file-encryptor
   ```

2. Build the application in release mode:

   ```bash
   cargo build --release
   ```

3. The binary will be available in `target/release/file-encryptor`.

## Usage

This application provides two primary operations: `encode` and `decode`.

### Syntax

```bash
file-encryptor {encode|decode} <input_path> <output_path>
```

- `{encode|decode}`: Specify the operation: `encode` to encrypt, `decode` to decrypt.
- `<input_path>`: Path to the input file (e.g., `./test.png`).
- `<output_path>`: Path to the output file (e.g., `./test.encrypted`).

### Examples

#### Encrypting a File

To encrypt `test.png` and save the result as `test.encrypted`:

```bash
file-encryptor encode ./test.png ./test.encrypted
```

#### Decrypting a File

To decrypt `test.encrypted` and save the result as `test.png`:

```bash
file-encryptor decode ./test.encrypted ./test.png
```

## Code Structure

- **src/main.rs**: Main application logic for parsing CLI arguments and calling encryption or decryption functions.
- **lib/lib.rs**: Contains the encryption function with AES-256 CBC mode and PKCS7 padding.
- **lib/lib.rs**: Contains the decryption function, which reverses the AES-256 CBC encryption with PKCS7 padding.
- **src/utils.rs**: Helper functions for key generation, IV generation, and handling files.

## Encryption Details

- **Algorithm**: AES-256 in CBC (Cipher Block Chaining) mode.
- **Padding**: PKCS7 padding for data that does not align with the AES block size (16 bytes).
- **Key**: A 32-byte key is derived from the user-provided password.
- **Embedded IV**: The 16-byte IV is randomly generated and embedded at the begining of the encrypted file.

## Dependencies

This project uses the following Rust crates:

- **[clap](https://docs.rs/clap/)**: For command-line argument parsing.
- **[aes](https://docs.rs/aes/)**: For AES encryption and decryption.
- **[cbc](https://docs.rs/cbc/)**: For CBC mode encryption.
- **[block-padding](https://docs.rs/block-padding/)**: For PKCS7 padding.
- **[rpassword](https://docs.rs/rpassword/)**: For securely reading passwords from the console.
- **[rand](https://docs.rs/rand/)**: For generating random initialization vectors (IVs).

## Future Enhancements

The following features could be implemented in future versions to enhance the functionality and usability of the application:

### 1. Support for Multiple Files and Directories

**Goal**: Allow users to specify multiple files or even an entire directory for encryption or decryption in a single command.

- **Batch Processing**: Enable the application to process multiple files by accepting a list of file paths or a directory path. When a directory is specified, the application would recursively encrypt or decrypt each file in the directory.
- **Output Directory**: Add an option to specify an output directory for encrypted/decrypted files, maintaining the original directory structure for ease of organization.

**Proposed CLI Syntax**:

```bash
file-encryptor encode ./file1.png ./file2.txt ./docs -o ./output_directory
```

This would encrypt `file1.png`, `file2.txt`, and all files within `./docs`, saving the results in `./output_directory`.

### 2. Custom Initialization Vector (IV) Input

**Goal**: Allow users to enter a custom IV for encryption to have more control over the process.

- **Custom IV Input**: Provide an option to manually specify a 16-byte IV. This is useful in scenarios where users require a specific IV for encryption.
- **IV Validation**: Ensure the IV is exactly 16 bytes, and display an error message if it doesn’t meet the requirement.
- **IV Output Option**: Add an option to output the generated IV to a file when it’s randomly generated, so users can reuse it for decryption.

**Proposed CLI Syntax**:

```bash
file-encryptor encode ./test.png ./test.encrypted --iv 1234567890abcdef
```

### 3. Configurable Encryption Algorithms and Modes

**Goal**: Offer more flexibility by allowing users to choose different encryption algorithms or modes (e.g., AES-GCM or AES-CTR).

- **Algorithm Choice**: Extend support to other AES modes such as GCM (for authenticated encryption) or CTR (for stream cipher mode), which may better suit certain use cases.
- **Configurable Padding**: Allow users to select the padding scheme (e.g., PKCS7, ISO7816) for better compatibility with other cryptographic systems.

### 4. Secure Password Storage and Key Management

**Goal**: Improve the security of password handling by integrating with secure key storage systems.

- **Key Derivation**: Implement a Key Derivation Function (KDF) such as PBKDF2, Argon2, or Scrypt to strengthen the user’s password, making brute-force attacks more difficult.
- **Password File**: Add support for securely reading passwords from a file or using a secure vault integration (e.g., HashiCorp Vault or AWS Secrets Manager) for enterprise-level security.

### 5. Logging and Progress Reporting

**Goal**: Provide feedback to users during batch encryption or decryption operations.

- **Progress Bar**: Show a progress bar when processing large files or multiple files, giving users visual feedback on the process.
- **Logging**: Implement logging for encryption/decryption operations, errors, and warnings. This can help with debugging and give users detailed feedback on successful operations.

### 6. Optional File Integrity Verification

**Goal**: Ensure file integrity after decryption to verify that the file has not been tampered with or corrupted.

- **Hash-Based Verification**: Calculate and store a hash (e.g., SHA-256) of the original file before encryption and compare it to the decrypted file’s hash.
- **Integrity Check Option**: Add an optional `--verify` flag to enable or disable integrity verification as needed.

---

These enhancements would make the application more robust, flexible, and user-friendly, supporting a broader range of use cases and providing stronger security for sensitive data.

## License

This project is licensed under the MIT License.
