# Build From Source 

### Dependencies 

1. Vyper Compiler (add to PATH),
2. Python,
3. Rust,
4. sqlx-cli: `cargo sqlx-cli install`

#### Rust 

Linux/Mac/WSL: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Windows: Download rustup-init.exe from `https://www.rust-lang.org/tools/install`

#### Vyper

`pip install vyper`

#### Initialize SQLite:

`cargo sqlx database create --database-url sqlite://deployer.db`

`cargo sqlx migrate run --database-url sqlite://deployer.db`

Windows: `move deployer.db ./src-tauri` | Linux / Mac: `mv deployer.db ./src-tauri`

#### Frontend (Svelte): 

`npm install`

Create a .env file in the project root with `DATABASE_URL = sqlite://deployer.db` on the first line. 

To run this software in non-release mode: `cargo tauri dev`. 

### Optional: tauri-cli (for bundling into single installer):
`cargo install tauri-cli` && `cargo tauri build`