# htmli [<img src="https://travis-ci.org/nathansizemore/htmli.svg?branch=master">][travis-badge]

Statically resolved include directives for HTML.

---

htmli allows you to break up your HTML pages into smaller modules. You simply
place a special include element:

``` html
<include src="dir/file.html" />
```

where you'd like the contents of `dir/file.html` to live, give htmli the path
of the HTML page, and file to write, and you're done.

Include elements are resolved relative to the entry file. So if you have the
following directory structure:

```
www/
    include/
        btn-menu.html
        nav.html
    main.html
```

And the following HTML

``` html
<!-- main.html  -->
<!doctype html>
<html>
    <head></head>
    <body>
        <include src="include/nav.html" />
    </body>
</html>

<!-- include/btn-menu.html  -->
<div id="btn-menu" class="btn">Push Me</div>

<!-- include/nav.html  -->
<div class="nav">
    <include src="include/menu-btn.html" />
</div>
```

htmli will first grab `main.html`, insert `include/nav.html`, and when it comes
across the include directive for `include/btn-menu.html`, it will resolve the
path from the directory `main.html` is currently in.

## Usage

```
htmli - Utility to statically resolve html-include directives.

Usage:
    htmli <file> [--minify] [--output=<f>]
    htmli (-h | --help)
    htmli --version

Options:
    -m --minify        Minifies output.
    -o --output=<f>    Direct output to file.

    -h --help          Show this screen.
    --version          Show version.
```

---

## Installation

First, install [Rust][rust-install-url].

###### Crates.io

```
$ cargo install htmli
```

###### Source

```
$ git clone https://github.com/nathansizemore/htmli
$ cd htmli
$ cargo install
```

---

### Author

Nathan Sizemore, nathanrsizemore@gmail.com

### License

htmli is available under the MPL-2.0 license. See the LICENSE file for
more info.



[travis-badge]: https://travis-ci.org/nathansizemore/keep
[rust-install-url]: https://www.rust-lang.org/en-US/install.html
