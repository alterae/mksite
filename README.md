# `mksite`, a static site generator

Turn a bunch of files into a different bunch of files, possibly resembling a website.

## Site structure

```
mksite.toml
out/
src/
	index.html
	about.md
	blog/
		2022-01-02.md
		2022-03-25.md
		special.html
static/
	favicon.ico
	style.css
template/
	_.html
	blog/
		_.html
		special.html
```

`mksite.toml` is the config file. it's used to specify transforms and site metadata.

The generated site goes in the `out/` directory.

`src` holds all the site content that needs to be transformed (stuff like markdown files).

`static` holds static files that are copied 1-1 to the output directory, unmodified.

`template` holds all the page template files. transformed page content is inserted into these templates.

## Config file

```toml
[metadata]
title = "Example Website"
base-url = "https://example.com"
copyright = "2022"

[transforms]
md.html = "md2html"
kdl.html = "./kdl-to-html.sh"
md.pdf = ["md2html", "pandoc -f html -t pdf"]
```

The `[metadata]` table is a set of key-value pairs that are made available to pages and templates.

The `[transforms]` table is a list of pairings in the format `in.out = "transform"`, where `in` is the extension of the input file, `out` the extension of the output file, and `transform` the transform to use for the given pairing.

A transform is simply an arbitrary command or command chain that takes the input file on standard input, and returns a transformed result on standard output. For example, an installed program that converts markdown to html, or a local shell script to apply syntax highlighting to code.

If no transform is provided for a given input type, files of that type will be written to the output directory as-is (though templating can still occur).

## Templates

Given a file with the destination path `out/foo/bar.html`, `mksite` will first look for a template at `template/foo/bar.html` (to allow for overriding templates on specific pages), and then `template/foo/_.html`, and finally `template/_.html`. Once found, the transformed page and relevant metadata will be inserted into the template, and the finished file written to the output directory. The file extension of a template should always match the extension of the output file, not the input. If no applicable template is found, the transformed page will be written as-is to the output directory.

## Order of operations

- Apply any templating and metadata.
- For every transform that takes this input format:
  - Pipe the file through the transform. (If no transform is applicable, pipe it through a phony transform with the same output extension that does nothing.
  - Insert the transformed file into the most relevant template that matches the transform's output extension. If no template is applicable, skip this step.
  - Write the templated file to the output directory.
