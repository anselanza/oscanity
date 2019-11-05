#!/usr/bin/env bash

# Exit if anything fails
set -euo pipefail

SCRIPT_PATH="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$SCRIPT_PATH/.."

if [ ! -f Cargo.toml ]; then
	echo "Unknown location. Please run this from the Oscanity repository. Abort."

	exit 1
fi

cat <<EOF
Welcome to Oscanity!

This script will download and install the necessary dependencies needed to
build Oscanity. This includes:
	* Rust (and the necessary components, e.g. rust-fmt, clippy)

If you'd prefer to install these dependencies yourself, please exit this script
now with Ctrl-C.

EOF

printf "Proceed with installing necessary dependencies? (Y/N) > "
read -e input
if [[ "$input" != "Y"* ]]; then
	echo "Exiting..."

	exit 0
fi

# Install Rust
echo "Installing Rust..."
if rustup --version &>/dev/null; then
	echo "Rust is already installed"
else
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain beta

	CARGO_ENV="$HOME/.cargo/env"

	source "$CARGO_ENV"
fi

# Run update in order to download and install the checked in toolchain
rustup update

# Add all the components that we need
rustup component add rustfmt
rustup component add clippy

cat <<EOF

Finished installing all dependencies.

You should now be able to build the project by running:
	source $HOME/.cargo/env
	cargo build
EOF
