# Recipe

Small Rust utility to manage a SQLite recipe database.

```
Usage: recipe [OPTIONS] <COMMAND>

Commands:
  list    List all recipes in the database
  create  TUI to create a new recipe in the database
  edit    TUI to edit a recipe
  print   Print out a recipe
  delete  Delete a recipe
  dump    Dump the entire database to SQL and markdown files
  tag     Manage tags
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --database-file <DATABASE_FILE>  Database file to use [default: recipes.db]
  -h, --help                           Print help
  -V, --version                        Print version
```

The database file can be specified using the `RECIPES_DATABASE_FILE` environment variable.

The recipe database can be dumped to a series of markdown files.
These files are designed to be hosted on GitHub pages.

```bash
recipe dump -d ./all-recipes
```

By default, this also dumps the entire recipe database to a `.sql` file inside the folder.
However, you need to have the `sqlite3` command in your path, or else this will not work.
This step can be skipped using the `--skip-sql` (or `-s`) flag.

```bash
recipe dump -d ./all-recipes --skip-sql
```
