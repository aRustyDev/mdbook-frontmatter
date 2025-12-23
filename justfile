root := `git rev-parse --show-toplevel`
docs := root + "/docs"
cfg := root + "/config.yaml"
doc_cfg := docs + "/book.toml"
templates := root + "/.github/templates"

mdbook := require("mdbook")
mustache := require("mustache")
rg := require("rg")

init:
    @"{{mustache}}" "{{templates}}/config.mustache" > "{{cfg}}"
    @"{{mustache}}" "{{cfg}}" "{{templates}}/book.mustache" > "{{doc_cfg}}"
    @yq -i 'git.url'
    @"{{mdbook}}" init --title "tmpl-mdbook-plugin" --ignore git .

build:
    @"{{mdbook}}" build docs/

# List all files in project related the id
ls-related id:
    @"{{rg}}" --sort=path -l -n "{{id}}" .aim/ .

find pattern file:
    @"{{rg}}" --sort=path -n "{{pattern}}" "{{file}}"
