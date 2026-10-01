# a ripgrep fork

This is a fork of ripgrep. ripgrep is a line-oriented search tool that
recursively searches the current directory for a regex pattern. See the
[upstream repository](https://github.com/BurntSushi/ripgrep) for full
documentation.

## about this fork

This fork tracks upstream ripgrep and adds a few focused conveniences:

- shorthand preview modes for `-M/--max-columns`, including a centered preview
  that keeps the first match visible;
- optional C-style escape interpretation in replacement strings;
- input path lists from files or stdin, with newline or NUL terminators;
- built-in file types for archives, compressed files and specific compressed
  tar formats; and
- support for internal hyphens in custom file type names.

Everything else behaves like upstream ripgrep unless noted below.

### input path lists

`--in <PATH|->` reads files and directories to search from a newline-terminated
list. `--in0 <PATH|->` uses NUL terminators, supporting filenames containing
newlines. Use `-` to read the list from stdin:

```console
$ rg --in paths.txt pattern
$ find src -print0 | rg --in0 - pattern
$ rg -l0 first-pattern | rg --in0 - second-pattern
```

Both options may be repeated and combined with positional paths. Each list is
expanded where its option appears; duplicates are retained. Entries behave
like command-line paths: relative paths resolve from the current working
directory, directories recurse, and explicitly named files bypass discovery
filters. They also work with `--files`.

`--in` accepts LF and CRLF terminators. All other whitespace and bytes are
preserved, including newlines under `--in0`. A final path without a terminator
is accepted. Empty records are errors; a trailing terminator is fine. A
zero-byte list contributes no paths. If no search paths remain, ripgrep
searches nothing and exits with status 1 instead of searching CWD or stdin.
NUL bytes in `--in` are errors. Unix filenames need not be UTF-8; other
platforms require UTF-8.

Lists are read completely before searching. A list entry of `-` searches
stdin; `./-` searches a literal filename. Stdin cannot supply a list and also
supply patterns or searchable contents, or supply more than one list.

### replacement escapes

`--replace-escapes` interprets escapes in the final `-r/--replace` string:

```console
$ rg --replace-escapes -r '$1\t$2' '(\w+) (\w+)'
```

Supported escapes are `\a` (alert), `\b` (backspace), `\f` (form-feed),
`\v` (vertical-tab), `\r`, `\t`, `\n`, `\\` (literal backslash), and `\R`
(CRLF on Windows, LF elsewhere, independently of `--crlf` or `--null-data`).
Numeric forms are:

- `\xHH`: exactly two hexadecimal digits producing one raw byte;
- `\N`, `\NN`, `\NNN`: one to three octal digits, consumed greedily, producing
  one raw byte modulo 256;
- `\uHHHH`, `\UHHHHHHHH`: exactly four or eight hexadecimal digits encoding
  a Unicode scalar as UTF-8. Surrogates and values above `10FFFF` are errors.

Additional digits after a complete escape are ordinary text. Raw byte escapes
may produce invalid UTF-8 without `--text` or `--binary`. JSON output uses its
existing base64 representation for replacement text that is not valid UTF-8.

Capture placeholders are recognized before decoding: `$1\x61` means capture 1
followed by `a`, while `$\x31` and `\x241` both produce literal `$1`. Use `$$`
for a literal dollar; `\$` is an error. Decoding is not recursive, and captured
contents are never decoded or subjected to escape validation.

Unknown or malformed escapes, a trailing backslash, or any escape producing
NUL (byte 00) cause a diagnostic and exit status 2 before searching. Neither
`--text` nor `--binary` bypasses these errors. Only the final replacement is
validated; the flag has no effect without a replacement.

Escape interpretation is disabled by default. An explicit
`--no-replace-escapes` anywhere in the configuration file or command line
disables interpretation and validation, even if `--replace-escapes` follows it.

### max-column preview suffixes

`-M/--max-columns` accepts an optional lowercase suffix after its numeric
value:

- `p` enables preview mode, equivalent to also passing
  `--max-columns-preview`;
- `c` enables preview mode and centers the preview around the first match.

For example:

```console
$ rg -M80p pattern
$ rg -M80c pattern
$ rg --max-columns 80c pattern
```

Preview windows preserve Unicode grapheme boundaries. Centered previews show
`[...]` when text was removed from the beginning of the line and retain the
usual suffix describing omitted text or additional matches.

Suffixes are lowercase only. A value of `0`, including `0p` or `0c`, disables
the max-column limit.

### archive and compression file types

The fork adds umbrella types for finding archive and compression-related
files:

- `archive` and `compressed` are aliases for the complete set;
- `just-archive` selects tar archives, compressed tar archives, ZIP, 7-Zip and
  RAR files;
- `just-compressed` selects standalone Brotli, bzip2, gzip, LZ4, LZMA, xz,
  Unix compress and Zstandard files; and
- `tar-compressed` selects all supported compressed tar filename forms.

Specific types are also available:

```text
7z             brotli        bzip2         gzip
lz4            lzma          rar           tar
tar-brotli     tar-bzip2     tar-gzip      tar-lz4
tar-lzip       tar-lzma      tar-pixz      tar-xz
tar-z          tar-zstd      xz            z
zip            zstd
```

These types work anywhere normal ripgrep type filters work:

```console
$ rg --files -tarchive
$ rg -z -tjust-compressed pattern
$ rg --files -ttar-zstd
```

Type filtering selects files by name. It does not recursively search container
formats such as ZIP, 7-Zip or RAR. As in upstream ripgrep, `-z/--search-zip`
can decompress supported single-stream compression formats, but it does not
turn archive containers into directory trees.

### hyphenated custom file types

Custom type names created with `--type-add` may contain internal hyphens:

```console
$ rg --type-add 'wat-lang:*.wat' -twat-lang pattern
```

Names must begin and end with a Unicode letter or number. Between them,
letters, numbers and hyphens are accepted; other punctuation is rejected.

## building

This fork requires Rust 1.96 or newer.

```console
$ cargo build --release
```

The resulting binary is `target/release/rg`.

## license

Like upstream ripgrep, this fork is dual-licensed under the MIT license or the
UNLICENSE.
