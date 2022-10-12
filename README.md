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

`mksite.toml` is the config file. it's used to specify processors and site metadata.

The generated site goes in the `out/` directory.

`src` holds all the site content that needs to be processed (stuff like markdown files).

`static` holds static files that are copied 1-1 to the output directory, unmodified.

`template` holds all the page template files. processed page content is inserted into these templates.

## Config file

```toml
[metadata]
title = "Example Website"
base-url = "https://example.com"
copyright = "2022"

[processors]
md.html = "md2html"
kdl.html = "process/kdl-to-html.sh"
md.pdf = ["md2html", "pandoc -f html -t pdf"]
```

The `[metadata]` table is a set of key-value pairs that are made available to pages and templates.

The `[processors]` table is a list of pairings in the format `in.out = "processor"`, where `in` is the extension of the input file, `out` the extension of the output file, and `processor` the processor to use for the given pairing.

A processor is simply an arbitrary command that takes the input file on standard input, and returns a processed result on standard output. For example, an installed program that converts markdown to html, or a local shell script to apply syntax highlighting to code.

If no processor is provided for a given input type, files of that type will be written to the output directory unprocessed (though templating can still occur).

## Templates

Given a file with the destination path `out/foo/bar.html`, `mksite` will first look for a template at `template/foo/bar.html` (to allow for overriding templates on specific pages), and then `template/foo/_.html`, and finally `template/_.html`. Once found, the processed page and relevant metadata will be inserted into the template, and the finished file written to the output directory. The file extension of a template should always match the extension of the output file, not the input. If no applicable template is found, the processed page will be written as-is to the output directory.

## Order of operations

- Apply any templating and metadata.
- For every processor that takes this input format:
  - Pipe the file through the processor. (If no processor is applicable, pipe it through a phony processor with the same output extension that does nothing.
  - Insert the processed file into the most relevant template that matches the processor's output extension. If no template is applicable, skip this step.
  - Write the templated file to the output directory.
