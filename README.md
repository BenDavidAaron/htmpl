# htmpl

pronounced: h-temple

A barebones static site generator for a personal project. 

## Usage

1. Make a templates dir
2. Make a base template with your CSS import, JS imports, `<head>` tag and `<body>` content you want on every page in your static site
3. run `htmpl /path/to/templates/dir /path/to/base.html /path/to/static-pages/`
4. Create an html file starting with the page title

reference the `test` dir for structure and content.

## Installation
1. `cargo build --relase`
2. `cp target/release/htmpl ./some/directory/in/your/path`