use clap::Parser;
use std::fs::File;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about = "Generates a PowerShell install script")]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value = "$env:USERPROFILE")]
    install_dir: String,
}

fn main() {
    let args = Args::parse();

    if !args.url.starts_with("http") {
        eprintln!("\n\x1b[1;31mERROR:\x1b[0m URL must start with http or https.");
        return;
    }

    let full_install_path = format!("{}\\{}", args.install_dir, args.name);

    let ps_script = format!(
        r#"# PowerShell Installer for {project_name}
$ProgressPreference = 'SilentlyContinue'
$url = "{download_url}"
$installDir = "{install_dir}"
$tempFile = "$env:TEMP\{project_name}_installer_temp"

# Determine file extension from URL
$extension = [System.IO.Path]::GetExtension($url)
$destFile = $tempFile + $extension

Write-Host "--- Starting Installation for {project_name} ---" -ForegroundColor Cyan

if (!(Test-Path $installDir)) {{
    Write-Host "[1/4] Creating directory: $installDir"
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
}}

Write-Host "[2/4] Downloading binaries..." -ForegroundColor Yellow
try {{
    Invoke-WebRequest -Uri $url -OutFile $destFile -ErrorAction Stop
}} catch {{
    Write-Host "ERROR: Failed to download." -ForegroundColor Red
    exit
}}

if ($extension -eq ".zip") {{
    Write-Host "[3/4] Extracting ZIP..." -ForegroundColor Yellow
    Expand-Archive -Path $destFile -DestinationPath $installDir -Force
    Remove-Item -Path $destFile
}} else {{
    Write-Host "[3/4] Moving binary..." -ForegroundColor Yellow
    Move-Item -Path $destFile -Destination "$installDir\{project_name}$extension" -Force
}}

# 4. Add to PATH (Permanent for User)
Write-Host "[4/4] Adding $installDir to User PATH..." -ForegroundColor Yellow
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -split ";" -notcontains $installDir) {{
    $newPath = "$currentPath;$installDir"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "PATH updated successfully. You will have to either restart your terminal
    or open a new one to be able to use {project_name}" -ForegroundColor Gray
}} else {{
    Write-Host "Directory already in PATH." -ForegroundColor Gray
}}

Write-Host "Done!" -ForegroundColor Green
Pause
"#,
        project_name = args.name,
        download_url = args.url,
        install_dir = full_install_path,
    );

    let filename = format!("{}-Installer.ps1", args.name.replace(" ", "_"));

    match save_to_file(&filename, &ps_script) {
        Ok(_) => println!("\n\x1b[1;32mSUCCESS:\x1b[0m Generated {}", filename),
        Err(e) => eprintln!("\n\x1b[1;31mERROR:\x1b[0m {}", e),
    }
}

fn save_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
