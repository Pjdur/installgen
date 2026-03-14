# installgen

Installgen is a tool to make install scripts for other tools. It generates a PowerShell script that can be used to download and install a binary or archive from a given URL. (Later it might support bash and other shells)

> AI note: I used AI to generate the PowerShell script because I'm not a PowerShell expert. The Rust code is majorly mine apart from some tweaks I used AI to do like adding a loop to the prompt if the input is empty. I've tested it a lot and it works well. If it doesn't work properly for you, please open an issue or a pull request to fix it.

## Installation

Installgen doesn't have a published version yet, but you can build it from source:

```bash
git clone https://github.com/Pjdur/installgen.git
cd installgen
cargo install --path .
```

## Usage

Run `installgen --name yourproject --url https://example.com/yourproject.[zip|exe] --install_dir $env:USERPROFILE`. The generated script will be saved as `[yourproject]-Installer.ps1` in the current directory.

## Notes

- I had to use double braces to escape the curly braces in the PowerShell script template. So if you see `{{` or `}}` in the Rust code, it's actually a single `{` or `}` in the generated PowerShell script.
- the generated Powershell script adds the install directory to the user's PATH (I thought that was useful because that's what I do when i install tools manually.)
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.