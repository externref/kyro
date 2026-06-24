#!/bin/bash

set -e

echo "building the kyro binary in release mode..."
cargo build --release

echo "creating the .kyro directory in the home folder..."
mkdir -p "$HOME/.kyro/lib"

echo "copying the compiled binary and the lib directory..."
cp target/release/kyro "$HOME/.kyro/"
if [ -d "lib" ]; then
  cp -r lib/* "$HOME/.kyro/lib/"
fi

echo "ensuring the binary has executable permissions..."
chmod +x "$HOME/.kyro/kyro"

shell_config=""
if [ -f "$HOME/.bashrc" ]; then
  shell_config="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
  shell_config="$HOME/.zshrc"
fi

if [ -n "$shell_config" ]; then
  if ! grep -q "KYRO_HOME" "$shell_config"; then
    echo "" >> "$shell_config"
    echo "export KYRO_HOME=\"\$HOME/.kyro\"" >> "$shell_config"
    echo "export PATH=\"\$KYRO_HOME:\$PATH\"" >> "$shell_config"
    echo "environment variables written to $shell_config"
  else
    echo "kyro variables are already configured in $shell_config"
  fi
else
  echo "warning: no standard shell profile (.bashrc or .zshrc) was detected."
  echo "manually add the following lines to your shell profile:"
  echo "export KYRO_HOME=\"\$HOME/.kyro\""
  echo "export PATH=\"\$KYRO_HOME:\$PATH\""
fi

echo "installation completed successfully."
echo "run 'source $shell_config' or restart your terminal to activate the changes."