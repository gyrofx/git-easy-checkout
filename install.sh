#!/bin/sh

# Define the GitHub repository
GITHUB_REPO="gyrofx/git-easy-checkout"
TOOL_NAME="git-easy-checkout"

# Determine the OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Check if the OS is macOS and adjust the OS variable accordingly
if [ "$OS" = "darwin" ]; then
  OS="macos"
fi

# Function to get the latest release version from GitHub API
get_latest_release_version() {
  LATEST_RELEASE=$(curl -s "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$LATEST_RELEASE" ]; then
    echo "Failed to fetch the latest release version from GitHub API. Aborting installation."
    exit 1
  fi
  RELEASE_VERSION="$LATEST_RELEASE"
}

# Function to download and install the binary
install_tool() {
  # Fetch the latest release version from GitHub API
  get_latest_release_version

  # Construct the URL to download the binary
  BINARY_URL="https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_VERSION}/${TOOL_NAME}-${OS}-${ARCH}"

  echo "Downloading ${TOOL_NAME} binary from GitHub..."
  if ! curl -L "$BINARY_URL" -o "/tmp/${TOOL_NAME}"; then
    echo "Failed to download the binary. Installation aborted."
    exit 1
  fi

  INSTALL_DIR="/usr/local/bin"

  # Check if admin privileges are required and prompt if necessary
  if [ ! -w "$INSTALL_DIR" ]; then
    echo "Admin privileges are required to install the tool."
    echo "Please enter your password to continue."
    sudo -k
    sudo echo "Thanks! Proceeding with installation."
  fi

  echo "Installing ${TOOL_NAME} to ${INSTALL_DIR}..."
  sudo mv "/tmp/${TOOL_NAME}" "$INSTALL_DIR/${TOOL_NAME}"
  sudo chmod +x "$INSTALL_DIR/${TOOL_NAME}"

  echo "Installation completed successfully."
}

# Call the install_tool function
install_tool
