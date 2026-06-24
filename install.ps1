$ErrorActionPreference = "Stop"

Write-Host "building the kyro binary in release mode..."
cargo build --release

Write-Host "creating the .kyro directory in the home folder..."
$kyro_home = Join-Path $HOME ".kyro"
$lib_dir = Join-Path $kyro_home "lib"
$null = New-Item -ItemType Directory -Force -Path $lib_dir

Write-Host "copying the compiled binary and the lib directory..."
Copy-Item -Path "target\release\kyro.exe" -Destination $kyro_home -Force
if (Test-Path "lib") {
    Copy-Item -Path "lib\*" -Destination $lib_dir -Recurse -Force
}

[System.Environment]::SetEnvironmentVariable("KYRO_HOME", $kyro_home, "User")

$user_path = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($user_path -notlike "*$kyro_home*") {
    $new_path = "$user_path;$kyro_home"
    [System.Environment]::SetEnvironmentVariable("Path", $new_path, "User")
    Write-Host "environment variables written to user profile"
} else {
    Write-Host "kyro variables are already configured"
}

Write-Host "installation completed successfully."
Write-Host "restart your terminal or shell to activate the changes."