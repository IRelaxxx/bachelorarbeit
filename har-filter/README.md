# har-filter

is a tool to filter har entries down to specific domains.

Usage:

`har-filter <input-file> <whitelist-file>`
where `input-file` is the filepath of the input har and `whitelist-file` is the filepath of a file that contains newline delimited regular expressions that conform to this [syntax](https://docs.rs/regex/1.3.9/regex/#syntax). An optional path for the output har that contains only the filtered entries can be supplied by `-o [output-filepath]`.
This tools print the used time of the filted entries calculated as `max(starttime + time elapsed) - lowest starttime`.

`har-filter pushconfig <input-file> <output-file>`
Generates a NGINX config file for HTTP2 Server push
