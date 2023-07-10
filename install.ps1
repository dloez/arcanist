function Remove-OldInstallation {
    Write-Host "Removing old installation..."
    $null = Remove-Item -LiteralPath $INSTALL_DIR -Force -Recurse
}

function Install-Arcanist {
    $null =  New-Item -ItemType Directory -Force -Path "${INSTALL_DIR}\bin"

    Write-Host "Downloading version ${VERSION}..."
    try {
        $null =  Invoke-WebRequest -URI $DOWNLOAD_URL -OutFile "${INSTALL_DIR}\bin\arcanist.exe"
    } catch {
        Write-Host "Failed to install arcanist"
        exit 1
    }
}


$VERSION = "0.1.0"
$INSTALL_DIR = "${env:localappdata}\arcanist"
$BINARY_NAME = "arcanist_${VERSION}_x86_64-pc-windows-msvc.exe"
$DOWNLOAD_URL = "https://github.com/dloez/arcanist/releases/download/v${VERSION}/${BINARY_NAME}"

if (Test-Path -Path $INSTALL_DIR) {
    Remove-OldInstallation
}

Install-Arcanist
echo "Done! To start using arcanist add the directory '${INSTALL_DIR}\bin' to the system/user path environment variable"
