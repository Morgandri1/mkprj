# mkprj
mkprj is a simple cli utility to make project creation more seamless. It allows you to create a project with most language's templates and then open it in your favorite editor.

# Installation
```bash
cargo install --git https://github.com/Morgandri1/mkprj
```

# Configuration
mkprj uses a json config file which you can easily interact with using the `mkprj config` command. The default config file is located at `~/.mkprj/config.json`.

The options are as follows:
- `--editor_command`: The editor to open the project in. This can be any command that opens a file in your editor. The default is `vim`.
- `--node_command`: Your javascript/node command. The default is `npm`, but you can change it to any one of `bun`, `yarn`, `deno`.

# Usage
mkprj is a fairly simple app to use. Simply run `mkprj <project_name> --<language>` to create a project. For example, to create a rust project, you would run `mkprj my_project --rust`.