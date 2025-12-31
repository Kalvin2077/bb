# bb (Bash Buddy)

**bb** acts as a central hub for your daily commands, providing interactive execution, smart memory, and a quick-access cheat sheet.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/bb.git

# Navigate into the directory
cd bb

# Build and install globally
cargo install --path .

```

## Configuration

`bb` looks for a `config.toml` file in `~/.config/bb/config.toml`. You can also place a `config.toml` in any project folder for project-specific commands.

### Example `config.toml`

```toml
[[tools]]
id = "git-amend"
name = "Git: Amend Last Commit"
command = "git commit --amend --no-edit"

[[cheats]]
name = "Docker Cleanup"
content = """
Remove all unused containers: docker container prune
Remove unused images: docker image prune -a
"""

```

## Usage

| Command | Description |
| --- | --- |
| `bb` or `bb run` | Open the interactive fuzzy-search menu to run a tool. |
| `bb cheat` | Search and display your cheat sheets. |
| `bb add` | (Optional) Add a new command to your config via CLI. |
| `bb --help` | Show help and usage details. |
