# a ripgrep fork

This is a fork of ripgrep. ripgrep is a line-oriented search tool that
recursively searches the current directory for a regex pattern. See the
[upstream repository](https://github.com/BurntSushi/ripgrep) for full
documentation.

## about this fork

This fork tracks upstream ripgrep and adds a few focused conveniences:

- shorthand preview modes for `-M/--max-columns`, including a centered preview
  that keeps the first match visible;
- built-in file types for archives, compressed files and specific compressed
  tar formats; and
- support for internal hyphens in custom file type names.

Everything else behaves like upstream ripgrep unless noted below.

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
