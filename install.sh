#!/bin/sh

get_platform() {
    case $(uname | tr '[:upper:]' '[:lower:]') in
        linux*)
            echo "unknown-linux-gnu"
            ;;
        darwin*)
            echo "apple-darwin"
            ;;
        mingw64_nt*)
            echo "unknown-linux-gnu"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

get_architecture() {
    case $(uname -m) in
        x86_64*)
            echo "x86_64"
            ;;
        aarch64*)
            echo "aarch64"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

environment_validation() {
    environment=$(get_platform)
    if [ $environment = "unknown" ]; then
        exit 1
    fi

    architecture=$(get_architecture)
    if [ $architecture = "unknown" ]; then
        exit 1
    fi

    if [ $(command -v wget >/dev/null 2>&1) ]; then
        exit 2
    fi

    echo "$environment:$architecture"
}

print_system_information() {
    echo "System environment: $(uname)"
    echo "CPU architecture: $(uname -m)"
}

remove_old_installation() {
    echo "Removing old installation..."
    rm -rf "$HOME/.arcanist"

    if [ -e "$HOME/.bashrc" ]; then
        sed -ie "s#$PROFILE_MOD_EXPORT##g" "$HOME/.bashrc"
        sed -i "s#$PROFILE_MOD_SOURCE##g" "$HOME/.bashrc"
    fi

    if [ -e "$HOME/.zshrc" ]; then
        sed -ie "s#$PROFILE_MOD_EXPORT##g" "$HOME/.zshrc"
        sed -ie "s#$PROFILE_MOD_SOURCE##g" "$HOME/.zshrc"
    fi

    if [ -e "$HOME/.fishrc" ]; then
        sed -ie "s#$PROFILE_MOD_EXPORT##g" "$HOME/.fishrc"
        sed -ie "s#$PROFILE_MOD_SOURCE##g" "$HOME/.fishrc"
    fi
}

install_arcanist() {
    mkdir -p "$HOME/.arcanist/bin"
    binary="arcanist_${VERSION}_${architecture}-${platform}"
    download_output=$(wget "${GITHUB_RELEASE_DOWNLOAD_URL}/${binary}" -O "$HOME/.arcanist/bin/arcanist")
    
    if [ $? -gt 0 ]; then
        exit $?
    fi

    chmod +x "$HOME/.arcanist/bin/arcanist"
    echo "$ARCANIST_SH" > "$HOME/.arcanist/arcanist.sh"

    if [ -e "$HOME/.bashrc" ]; then
        echo "$PROFILE_MOD_EXPORT" >> "$HOME/.bashrc"
        echo "$PROFILE_MOD_SOURCE" >> "$HOME/.bashrc"
    fi

    if [ -e "$HOME/.zshrc" ]; then
        echo "$PROFILE_MOD_EXPORT" >> "$HOME/.zshrc"
        echo "$PROFILE_MOD_SOURCE" >> "$HOME/.zshrc"
    fi

    if [ -e "$HOME/.fishrc" ]; then
        echo "$PROFILE_MOD_EXPORT" >> "$HOME/.fishrc"
        echo "$PROFILE_MOD_SOURCE" >> "$HOME/.fishrc"
    fi
}


VERSION="0.1.0"
GITHUB_REPOSITORY_BASE_URL="https://github.com/dloez"
GITHUB_RELEASES_BASE_URL="${GITHUB_REPOSITORY_BASE_URL}/arcanist/releases"
GITHUB_RELEASE_URL="${GITHUB_RELEASES_BASE_URL}/tag/${VERSION}"
GITHUB_RELEASE_DOWNLOAD_URL="${GITHUB_RELEASES_BASE_URL}/download/v${VERSION}"
GITHUB_ISSUES_NEW_URL="${GITHUB_REPOSITORY_BASE_URL}/issues/new"

PROFILE_MOD_EXPORT='export ARCANIST_DIR="$HOME/.arcanist"'
PROFILE_MOD_SOURCE='[ -s "$HOME/.arcanist/arcanist.sh" ] && source "$HOME/.arcanist/arcanist.sh"'

ARCANIST_SH='export PATH=$PATH:$HOME/.arcanist/bin'

environment=$(environment_validation)
case $? in
    1)
        echo "The installation script is not able to determine your environment or \
is not currently supported. Please create an issue in ${GITHUB_ISSUES_NEW_URL} with the following information:"
        print_system_information
        exit 1
        ;;
    2)
        echo "The tool wget is required. Please check the 'Install method: Automatic script' - 'Requirements' section \
in ${GITHUB_RELEASE_URL}"
        exit 1
        ;;
esac

platform=$(echo $environment | cut -d : -f 1)
architecture=$(echo $environment | cut -d : -f 2)

if [ -e "$HOME/.arcanist" ]; then
    remove_old_installation
fi

install_arcanist
if [ $? -gt 0 ]; then
    echo "Failed to install arcanist"
fi
