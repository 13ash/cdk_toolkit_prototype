# Resource Billing CLI

Resource Billing CLI is a command-line tool that reads and displays AWS resource billing information from a JSON file. This tool provides an easy way to review resource usage and costs, including a prompt to confirm deployment actions.

## Features

- Read and display AWS resource billing information from a JSON file.
- Color-coded output for better readability.
- Confirmation prompt for deployment actions.

## Installation

1. **Install Rust:**

   If you don't have Rust installed, you can install it using `rustup`:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

   Follow the on-screen instructions to complete the installation. Once installed, ensure your `PATH` environment variable is correctly set up by restarting your terminal or running:

    ```sh
    source $HOME/.cargo/env
    ```

2. **Clone the repository:**

    ```sh
    git clone https://github.com/yourusername/resource-billing-cli.git
    cd resource-billing-cli
    ```

3. **Install the required dependencies and build the project:**

    cargo build

## Usage

1.**Run the CLI tool:**

    cargo run --display --input src/cf_template.json


2. **Follow the prompt** to decide whether to deploy:

    ```sh
    Please visit your billing console for more details https://console.aws.amazon.com/console/home?nc2=h_ct&src=header-signin
    Do you want to deploy? [y/N]
    ```

## Dependencies

- [clap](https://crates.io/crates/clap) for command-line argument parsing.
- [serde](https://crates.io/crates/serde) for JSON deserialization.
- [colored](https://crates.io/crates/colored) for color-coded output.
- [dialoguer](https://crates.io/crates/dialoguer) for interactive prompts.
