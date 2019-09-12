# Package modifications controller

Go to the `working_repository` directory and initialize a git repository.

If you're in a git repository, this program takes one or two commit ids and prints the packages (directories with a package.json) that changed between them.

Pass a `--reference` (`-r`) argument to indicate the oldest commit, and an optional `--last` (`-l`) for the most recent one (`HEAD` by default).
