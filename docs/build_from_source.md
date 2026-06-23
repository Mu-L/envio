# Build from source

1. **Verify that you have Rust installed:**

   ```sh
   rustc --version
   ```

   If Rust is not installed, follow the instructions on the [Official Rust website](https://www.rust-lang.org/tools/install).

2. **Install dependencies:**

   Ensure the `gpg` CLI tool (GnuPG) is installed if you plan to use `gpg` encryption.

3. **Clone the repository:**

   ```sh
   git clone https://github.com/humblepenguinn/envio.git
   cd envio
   ```

4. **Build the project:**

   ```sh
   cargo build
   ```

5. **Verify the build:**

   ```sh
   cargo run -- version
   ```
