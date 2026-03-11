#!/bin/bash
set -e

# ── Configuration ─────────────────────────────────────────────────────
REPO="rainday/claude-statusline-rust"
INSTALL_DIR="$HOME/.claude/bin"
BINARY_NAME="claude-statusline-rs"
SETTINGS_FILE="$HOME/.claude/settings.json"
CONFIG_DIR="$HOME/.claude/statusline-rs"

# ── Helpers ───────────────────────────────────────────────────────────

info()  { echo "  → $*"; }
error() { echo "ERROR: $*" >&2; exit 1; }

detect_target() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
                aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
                *)       error "Unsupported Linux architecture: $arch" ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)  TARGET="x86_64-apple-darwin" ;;
                arm64)   TARGET="aarch64-apple-darwin" ;;
                *)       error "Unsupported macOS architecture: $arch" ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            TARGET="x86_64-pc-windows-msvc"
            ;;
        *)
            error "Unsupported OS: $os"
            ;;
    esac
}

is_windows() {
    [[ "$TARGET" == *"windows"* ]]
}

# Convert MSYS/Cygwin paths to Windows native paths for node/python
to_native_path() {
    if is_windows && command -v cygpath &>/dev/null; then
        cygpath -w "$1"
    else
        echo "$1"
    fi
}

binary_path() {
    if is_windows; then
        echo "${INSTALL_DIR}/${BINARY_NAME}.exe"
    else
        echo "${INSTALL_DIR}/${BINARY_NAME}"
    fi
}

# Update settings.json — merge statusLine key using node or python3.
# $1 = action: "add" or "remove"
# $2 = binary path (only needed for "add")
update_settings() {
    local action="$1"
    local bin_path="$2"

    if [[ "$action" == "add" ]]; then
        local js_code node_code py_code

        # Node.js script — works with plain node (no dependencies)
        read -r -d '' node_code <<'NODESCRIPT' || true
const fs = require("fs");
const file = process.argv[1];
const binPath = process.argv[2];
let data = {};
try { data = JSON.parse(fs.readFileSync(file, "utf8")); } catch {}
data.statusLine = { type: "command", command: binPath };
fs.writeFileSync(file, JSON.stringify(data, null, 2) + "\n");
NODESCRIPT

        read -r -d '' py_code <<'PYSCRIPT' || true
import json, sys, os
file, bin_path = sys.argv[1], sys.argv[2]
data = {}
if os.path.isfile(file):
    with open(file) as f:
        data = json.load(f)
data["statusLine"] = {"type": "command", "command": bin_path}
with open(file, "w") as f:
    json.dump(data, f, indent=2)
    f.write("\n")
PYSCRIPT

        mkdir -p "$(dirname "$SETTINGS_FILE")"

        local native_settings native_bin
        native_settings="$(to_native_path "$SETTINGS_FILE")"
        native_bin="$(to_native_path "$bin_path")"

        if command -v node &>/dev/null; then
            node -e "$node_code" "$native_settings" "$native_bin"
        elif command -v python3 &>/dev/null; then
            python3 -c "$py_code" "$native_settings" "$native_bin"
        else
            echo ""
            echo "  Could not auto-configure settings (node/python3 not found)."
            echo "  Please add the following to $SETTINGS_FILE manually:"
            echo ""
            echo "    \"statusLine\": { \"type\": \"command\", \"command\": \"$bin_path\" }"
            echo ""
            return
        fi
        info "Updated $SETTINGS_FILE"

    elif [[ "$action" == "remove" ]]; then
        [ -f "$SETTINGS_FILE" ] || return 0

        read -r -d '' node_code <<'NODESCRIPT' || true
const fs = require("fs");
const file = process.argv[1];
try {
    const data = JSON.parse(fs.readFileSync(file, "utf8"));
    delete data.statusLine;
    fs.writeFileSync(file, JSON.stringify(data, null, 2) + "\n");
} catch {}
NODESCRIPT

        read -r -d '' py_code <<'PYSCRIPT' || true
import json, sys, os
file = sys.argv[1]
if not os.path.isfile(file):
    sys.exit(0)
with open(file) as f:
    data = json.load(f)
data.pop("statusLine", None)
with open(file, "w") as f:
    json.dump(data, f, indent=2)
    f.write("\n")
PYSCRIPT

        local native_settings
        native_settings="$(to_native_path "$SETTINGS_FILE")"

        if command -v node &>/dev/null; then
            node -e "$node_code" "$native_settings"
        elif command -v python3 &>/dev/null; then
            python3 -c "$py_code" "$native_settings"
        else
            echo ""
            echo "  Could not auto-remove settings (node/python3 not found)."
            echo "  Please remove the \"statusLine\" key from $SETTINGS_FILE manually."
            echo ""
            return
        fi
        info "Cleaned $SETTINGS_FILE"
    fi
}

# ── Install ───────────────────────────────────────────────────────────

do_install() {
    echo "Installing ${BINARY_NAME}..."
    echo ""

    detect_target
    info "Detected target: $TARGET"

    # Fetch latest release tag
    info "Fetching latest release from GitHub..."
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local release_json
    release_json="$(curl -fsSL "$api_url")" || error "Failed to fetch release info from $api_url"

    local version
    version="$(echo "$release_json" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')"
    [ -n "$version" ] || error "Could not determine latest version"
    info "Latest version: $version"

    # Determine asset name and download URL
    local asset_name download_url
    if is_windows; then
        asset_name="${BINARY_NAME}-${version}-${TARGET}.zip"
    else
        asset_name="${BINARY_NAME}-${version}-${TARGET}.tar.gz"
    fi

    download_url="https://github.com/${REPO}/releases/download/${version}/${asset_name}"
    info "Downloading $asset_name..."

    local tmpdir
    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    curl -fsSL -o "${tmpdir}/${asset_name}" "$download_url" \
        || error "Download failed: $download_url"

    # Extract
    info "Extracting..."
    if is_windows; then
        (cd "$tmpdir" && unzip -qo "$asset_name")
    else
        tar -xzf "${tmpdir}/${asset_name}" -C "$tmpdir"
    fi

    # Install binary
    mkdir -p "$INSTALL_DIR"
    local bin
    bin="$(binary_path)"

    local extracted_bin
    if is_windows; then
        extracted_bin="$(find "$tmpdir" -name "${BINARY_NAME}.exe" -type f | head -1)"
    else
        extracted_bin="$(find "$tmpdir" -name "${BINARY_NAME}" -type f | head -1)"
    fi
    [ -n "$extracted_bin" ] || error "Binary not found in archive"

    cp "$extracted_bin" "$bin"
    chmod +x "$bin"
    info "Installed to $bin"

    # Configure settings.json
    update_settings "add" "$bin"

    echo ""
    echo "Done! ${BINARY_NAME} ${version} installed."
    echo "Binary: $bin"
    echo "Restart Claude Code to see the statusline."
}

# ── Uninstall ─────────────────────────────────────────────────────────

do_uninstall() {
    echo "Uninstalling ${BINARY_NAME}..."
    echo ""

    detect_target

    # Remove binary
    local bin
    bin="$(binary_path)"
    if [ -f "$bin" ]; then
        rm -f "$bin"
        info "Removed $bin"
    else
        info "Binary not found at $bin (already removed?)"
    fi

    # Clean settings.json
    update_settings "remove"

    # Remove config directory
    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        info "Removed $CONFIG_DIR"
    else
        info "Config dir not found at $CONFIG_DIR (already removed?)"
    fi

    echo ""
    echo "Done! ${BINARY_NAME} has been uninstalled."
}

# ── Main ──────────────────────────────────────────────────────────────

case "${1:-install}" in
    install)   do_install   ;;
    uninstall) do_uninstall ;;
    *)
        echo "Usage: $0 [install|uninstall]"
        exit 1
        ;;
esac
