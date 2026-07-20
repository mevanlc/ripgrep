#[rustfmt::skip]
const TAR_COMPRESSED_GLOBS: &[&str] = &[
    "*.tar.gz", "*.tgz", "*.tar.bz2", "*.tbz2", "*.tbz", "*.tb2",
    "*.tar.xz", "*.txz", "*.tpxz", "*.tar.pxz", "*.tar.zst", "*.tzst",
    "*.tar.Z", "*.taz", "*.tar.lz", "*.tlz", "*.tar.lzma", "*.tar.lzm",
    "*.tar.lz4", "*.tar.br",
];

#[rustfmt::skip]
const JUST_ARCHIVE_GLOBS: &[&str] = &[
    "*.tar.gz", "*.tgz", "*.tar.bz2", "*.tbz2", "*.tbz", "*.tb2",
    "*.tar.xz", "*.txz", "*.tpxz", "*.tar.pxz", "*.tar.zst", "*.tzst",
    "*.tar.Z", "*.taz", "*.tar.lz", "*.tlz", "*.tar.lzma", "*.tar.lzm",
    "*.tar.lz4", "*.tar.br", "*.tar", "*.zip", "*.7z", "*.rar",
];

#[rustfmt::skip]
const JUST_COMPRESSED_GLOBS: &[&str] = &[
    "*.br", "*.bz2", "*.gz", "*.lz4", "*.lzma", "*.xz", "*.Z", "*.zst",
    "*.zstd",
];

#[rustfmt::skip]
const ARCHIVE_GLOBS: &[&str] = &[
    "*.br", "*.bz2", "*.gz", "*.lz4", "*.lzma", "*.xz", "*.Z", "*.zst",
    "*.zstd", "*.tar.gz", "*.tgz", "*.tar.bz2", "*.tbz2", "*.tbz",
    "*.tb2", "*.tar.xz", "*.txz", "*.tpxz", "*.tar.pxz", "*.tar.zst",
    "*.tzst", "*.tar.Z", "*.taz", "*.tar.lz", "*.tlz", "*.tar.lzma",
    "*.tar.lzm", "*.tar.lz4", "*.tar.br", "*.tar", "*.zip", "*.7z",
    "*.rar",
];

/// This list represents the default file types that ripgrep ships with. In
/// general, any file format is fair game, although it should generally be
/// limited to reasonably popular open formats. For other cases, you can add
/// types to each invocation of ripgrep with the '--type-add' flag.
///
/// If you would like to add or improve this list, please file a PR:
/// <https://github.com/BurntSushi/ripgrep>.
///
/// Please try to keep this list sorted lexicographically and wrapped to 79
/// columns (inclusive).
#[rustfmt::skip]
pub(crate) const DEFAULT_TYPES: &[(&[&str], &[&str])] = &[
    (&["7z"], &["*.7z"]),
    (&["ada"], &["*.adb", "*.ads"]),
    (&["agda"], &["*.agda", "*.lagda"]),
    (&["aidl"], &["*.aidl"]),
    (&["alire"], &["alire.toml"]),
    (&["amake"], &["*.mk", "*.bp"]),
    (&["archive"], ARCHIVE_GLOBS),
    (&["asciidoc"], &["*.adoc", "*.asc", "*.asciidoc"]),
    (&["asm"], &["*.asm", "*.s", "*.S"]),
    (&["asp"], &[
        "*.aspx", "*.aspx.cs", "*.aspx.vb", "*.ascx", "*.ascx.cs",
        "*.ascx.vb", "*.asp"
    ]),
    (&["ats"], &["*.ats", "*.dats", "*.sats", "*.hats"]),
    (&["avro"], &["*.avdl", "*.avpr", "*.avsc"]),
    (&["awk"], &["*.awk"]),
    (&["bat", "batch"], &["*.bat"]),
    (&["bazel"], &[
        "*.bazel", "*.bzl", "*.BUILD", "*.bazelrc", "BUILD", "MODULE.bazel",
        "WORKSPACE", "WORKSPACE.bazel", "WORKSPACE.bzlmod",
    ]),
    (&["bitbake"], &["*.bb", "*.bbappend", "*.bbclass", "*.conf", "*.inc"]),
    (&["boxlang"], &["*.bx", "*.bxm", "*.bxs"]),
    (&["brotli"], &["*.br"]),
    (&["buildstream"], &["*.bst"]),
    (&["bzip2"], &["*.bz2"]),
    (&["c"], &["*.[chH]", "*.[chH].in", "*.cats"]),
    (&["cabal"], &["*.cabal"]),
    (&["candid"], &["*.did"]),
    (&["carp"], &["*.carp"]),
    (&["cbor"], &["*.cbor"]),
    (&["ceylon"], &["*.ceylon"]),
    (&["cfml"], &["*.cfc", "*.cfm"]),
    (&["clojure"], &["*.clj", "*.cljc", "*.cljs", "*.cljx"]),
    (&["cmake"], &["*.cmake", "CMakeLists.txt"]),
    (&["cmd"], &["*.bat", "*.cmd"]),
    (&["cml"], &["*.cml"]),
    (&["coffeescript"], &["*.coffee"]),
    (&["compressed"], ARCHIVE_GLOBS),
    (&["config"], &["*.cfg", "*.conf", "*.config", "*.ini"]),
    (&["container"], &["*Containerfile*", "*Dockerfile*"]),
    (&["coq"], &["*.v"]),
    (&["cpp"], &[
        "*.[ChH]", "*.cc", "*.[ch]pp", "*.[ch]xx", "*.hh",  "*.inl",
        "*.[ChH].in", "*.cc.in", "*.[ch]pp.in", "*.[ch]xx.in", "*.hh.in",
    ]),
    (&["creole"], &["*.creole"]),
    (&["crystal"], &["Projectfile", "*.cr", "*.ecr", "shard.yml"]),
    (&["cs"], &["*.cs"]),
    (&["csharp"], &["*.cs"]),
    (&["cshtml"], &["*.cshtml"]),
    (&["csproj"], &["*.csproj"]),
    (&["css"], &["*.css", "*.scss"]),
    (&["csv"], &["*.csv"]),
    (&["cuda"], &["*.cu", "*.cuh"]),
    (&["cython"], &["*.pyx", "*.pxi", "*.pxd"]),
    (&["d"], &["*.d"]),
    (&["dart"], &["*.dart"]),
    (&["devicetree"], &["*.dts", "*.dtsi", "*.dtso"]),
    (&["dhall"], &["*.dhall"]),
    (&["diff"], &["*.patch", "*.diff"]),
    (&["dita"], &["*.dita", "*.ditamap", "*.ditaval"]),
    (&["docker"], &["*Dockerfile*"]),
    (&["dockercompose"], &["docker-compose.yml", "docker-compose.*.yml"]),
    (&["dts"], &["*.dts", "*.dtsi"]),
    (&["dvc"], &["Dvcfile", "*.dvc"]),
    (&["ebuild"], &["*.ebuild", "*.eclass"]),
    (&["edn"], &["*.edn"]),
    (&["elisp"], &["*.el"]),
    (&["elixir"], &["*.ex", "*.eex", "*.exs", "*.heex", "*.leex", "*.livemd"]),
    (&["elm"], &["*.elm"]),
    (&["erb"], &["*.erb"]),
    (&["erlang"], &["*.erl", "*.hrl"]),
    (&["fennel"], &["*.fnl"]),
    (&["fidl"], &["*.fidl"]),
    (&["fish"], &["*.fish"]),
    (&["flatbuffers"], &["*.fbs"]),
    (&["fortran"], &[
        "*.f", "*.F", "*.f77", "*.F77", "*.pfo",
        "*.f90", "*.F90", "*.f95", "*.F95",
    ]),
    (&["fsharp"], &["*.fs", "*.fsx", "*.fsi"]),
    (&["fut"], &["*.fut"]),
    (&["gap"], &["*.g", "*.gap", "*.gi", "*.gd", "*.tst"]),
    (&["gdscript"], &["*.gd"]),
    (&["gleam"], &["*.gleam"]),
    (&["gn"], &["*.gn", "*.gni"]),
    (&["go"], &["*.go"]),
    (&["gprbuild"], &["*.gpr"]),
    (&["gradle"], &[
        "*.gradle", "*.gradle.kts", "gradle.properties", "gradle-wrapper.*",
        "gradlew", "gradlew.bat",
    ]),
    (&["graphql"], &["*.graphql", "*.graphqls"]),
    (&["groovy"], &["*.groovy", "*.gradle"]),
    (&["gzip"], &["*.gz"]),
    (&["h"], &["*.h", "*.hh", "*.hpp"]),
    (&["haml"], &["*.haml"]),
    (&["hare"], &["*.ha"]),
    (&["haskell"], &["*.hs", "*.lhs", "*.cpphs", "*.c2hs", "*.hsc"]),
    (&["hbs"], &["*.hbs"]),
    (&["hs"], &["*.hs", "*.lhs"]),
    (&["html"], &["*.htm", "*.html", "*.ejs"]),
    (&["hurl"], &["*.hurl"]),
    (&["hy"], &["*.hy"]),
    (&["idris"], &["*.idr", "*.lidr"]),
    (&["janet"], &["*.janet"]),
    (&["java"], &["*.java", "*.jsp", "*.jspx", "*.properties"]),
    (&["jinja"], &["*.j2", "*.jinja", "*.jinja2"]),
    (&["jl"], &["*.jl"]),
    (&["js"], &["*.js", "*.jsx", "*.vue", "*.cjs", "*.mjs"]),
    (&["json"], &["*.json", "composer.lock", "*.sarif"]),
    (&["jsonl"], &["*.jsonl"]),
    (&["julia"], &["*.jl"]),
    (&["jupyter"], &["*.ipynb", "*.jpynb"]),
    (&["just-archive"], JUST_ARCHIVE_GLOBS),
    (&["just-compressed"], JUST_COMPRESSED_GLOBS),
    (&["k"], &["*.k"]),
    (&["kconfig"], &["Kconfig", "Kconfig.*"]),
    (&["kotlin"], &["*.kt", "*.kts"]),
    (&["lean"], &["*.lean"]),
    (&["less"], &["*.less"]),
    (&["license"], &[
        // General
        "COPYING", "COPYING[.-]*",
        "COPYRIGHT", "COPYRIGHT[.-]*",
        "EULA", "EULA[.-]*",
        "licen[cs]e", "licen[cs]e.*",
        "LICEN[CS]E", "LICEN[CS]E[.-]*", "*[.-]LICEN[CS]E*",
        "NOTICE", "NOTICE[.-]*",
        "PATENTS", "PATENTS[.-]*",
        "UNLICEN[CS]E", "UNLICEN[CS]E[.-]*",
        // GPL (gpl.txt, etc.)
        "agpl[.-]*",
        "gpl[.-]*",
        "lgpl[.-]*",
        // Other license-specific (APACHE-2.0.txt, etc.)
        "AGPL-*[0-9]*",
        "APACHE-*[0-9]*",
        "BSD-*[0-9]*",
        "CC-BY-*",
        "GFDL-*[0-9]*",
        "GNU-*[0-9]*",
        "GPL-*[0-9]*",
        "LGPL-*[0-9]*",
        "MIT-*[0-9]*",
        "MPL-*[0-9]*",
        "OFL-*[0-9]*",
    ]),
    (&["lilypond"], &["*.ly", "*.ily"]),
    (&["lisp"], &["*.el", "*.jl", "*.lisp", "*.lsp", "*.sc", "*.scm"]),
    (&["llvm"], &["*.ll"]),
    (&["lock"], &["*.lock", "package-lock.json"]),
    (&["log"], &["*.log"]),
    (&["lua"], &["*.lua"]),
    (&["lz4"], &["*.lz4"]),
    (&["lzma"], &["*.lzma"]),
    (&["m4"], &["*.ac", "*.m4"]),
    (&["make"], &[
        "[Gg][Nn][Uu]makefile", "[Mm]akefile",
        "[Gg][Nn][Uu]makefile.am", "[Mm]akefile.am",
        "[Gg][Nn][Uu]makefile.in", "[Mm]akefile.in",
        "Makefile.*",
        "*.mk", "*.mak"
    ]),
    (&["mako"], &["*.mako", "*.mao"]),
    (&["man"], &["*.[0-9lnpx]", "*.[0-9][cEFMmpSx]"]),
    (&["markdown", "md"], &[
        "*.markdown",
        "*.md",
        "*.mdown",
        "*.mdwn",
        "*.mkd",
        "*.mkdn",
        "*.mdx",
    ]),
    (&["matlab"], &["*.m"]),
    (&["meson"], &["meson.build", "meson_options.txt", "meson.options"]),
    (&["minified"], &["*.min.html", "*.min.css", "*.min.js"]),
    (&["mint"], &["*.mint"]),
    (&["mk"], &["mkfile"]),
    (&["ml"], &["*.ml"]),
    (&["motoko"], &["*.mo"]),
    (&["msbuild"], &[
        "*.csproj", "*.fsproj", "*.vcxproj", "*.proj", "*.props", "*.targets",
        "*.sln", "*.slnf"
    ]),
    (&["nim"], &["*.nim", "*.nimf", "*.nimble", "*.nims"]),
    (&["nix"], &["*.nix"]),
    (&["objc"], &["*.h", "*.m"]),
    (&["objcpp"], &["*.h", "*.mm"]),
    (&["ocaml"], &["*.ml", "*.mli", "*.mll", "*.mly"]),
    (&["org"], &["*.org", "*.org_archive"]),
    (&["pants"], &["BUILD"]),
    (&["pascal"], &["*.pas", "*.dpr", "*.lpr", "*.pp", "*.inc"]),
    (&["pdf"], &["*.pdf"]),
    (&["perl"], &["*.perl", "*.pl", "*.PL", "*.plh", "*.plx", "*.pm", "*.t"]),
    (&["php"], &[
        // note that PHP 6 doesn't exist
        // See: https://wiki.php.net/rfc/php6
        "*.php", "*.php3", "*.php4", "*.php5", "*.php7", "*.php8",
        "*.pht", "*.phtml"
    ]),
    (&["po"], &["*.po"]),
    (&["pod"], &["*.pod"]),
    (&["postscript"], &["*.eps", "*.ps"]),
    (&["prolog"], &["*.pl", "*.pro", "*.prolog", "*.P"]),
    (&["protobuf"], &["*.proto"]),
    (&["ps"], &["*.cdxml", "*.ps1", "*.ps1xml", "*.psd1", "*.psm1"]),
    (&["puppet"], &["*.epp", "*.erb", "*.pp", "*.rb"]),
    (&["purs"], &["*.purs"]),
    (&["py", "python"], &["*.py", "*.pyi"]),
    (&["qmake"], &["*.pro", "*.pri", "*.prf"]),
    (&["qml"], &["*.qml"]),
    (&["qrc"], &["*.qrc"]),
    (&["qui"], &["*.ui"]),
    (&["r"], &["*.R", "*.r", "*.Rmd", "*.rmd", "*.Rnw", "*.rnw"]),
    (&["racket"], &["*.rkt"]),
    (&["raku"], &[
        "*.raku", "*.rakumod", "*.rakudoc", "*.rakutest",
        "*.p6", "*.pl6", "*.pm6"
    ]),
    (&["rar"], &["*.rar"]),
    (&["rdoc"], &["*.rdoc"]),
    (&["readme"], &["README*", "*README"]),
    (&["reasonml"], &["*.re", "*.rei"]),
    (&["red"], &["*.r", "*.red", "*.reds"]),
    (&["rescript"], &["*.res", "*.resi"]),
    (&["robot"], &["*.robot"]),
    (&["rst"], &["*.rst"]),
    (&["ruby"], &[
        // Idiomatic files
        "config.ru", "Gemfile", ".irbrc", "Rakefile",
        // Extensions
        "*.gemspec", "*.rb", "*.rbw", "*.rake"
    ]),
    (&["rust"], &["*.rs"]),
    (&["sass"], &["*.sass", "*.scss"]),
    (&["scala"], &["*.scala", "*.sbt"]),
    (&["scdoc"], &["*.scd", "*.scdoc"]),
    (&["seed7"], &["*.sd7", "*.s7i"]),
    (&["sh"], &[
        // Portable/misc. init files
        ".env", ".login", ".logout", ".profile", "profile",
        // bash-specific init files
        ".bash_login", "bash_login",
        ".bash_logout", "bash_logout",
        ".bash_profile", "bash_profile",
        ".bashrc", "bashrc", "*.bashrc",
        // csh-specific init files
        ".cshrc", "*.cshrc",
        // ksh-specific init files
        ".kshrc", "*.kshrc",
        // tcsh-specific init files
        ".tcshrc",
        // zsh-specific init files
        ".zshenv", "zshenv",
        ".zlogin", "zlogin",
        ".zlogout", "zlogout",
        ".zprofile", "zprofile",
        ".zshrc", "zshrc",
        // Extensions
        "*.bash", "*.csh", "*.env", "*.ksh", "*.sh", "*.tcsh", "*.zsh",
    ]),
    (&["slim"], &["*.skim", "*.slim", "*.slime"]),
    (&["smarty"], &["*.tpl"]),
    (&["sml"], &["*.sml", "*.sig"]),
    (&["solidity"], &["*.sol"]),
    (&["soy"], &["*.soy"]),
    (&["spark"], &["*.spark"]),
    (&["spec"], &["*.spec"]),
    (&["sql"], &["*.sql", "*.psql"]),
    (&["ssa"], &["*.ssa"]),
    (&["stylus"], &["*.styl"]),
    (&["sv"], &["*.v", "*.vg", "*.sv", "*.svh", "*.h"]),
    (&["svelte"], &["*.svelte", "*.svelte.ts"]),
    (&["svg"], &["*.svg"]),
    (&["swift"], &["*.swift"]),
    (&["swig"], &["*.def", "*.i"]),
    (&["systemd"], &[
        "*.automount", "*.conf", "*.device", "*.link", "*.mount", "*.path",
        "*.scope", "*.service", "*.slice", "*.socket", "*.swap", "*.target",
        "*.timer",
    ]),
    (&["tar"], &["*.tar"]),
    (&["tar-brotli"], &["*.tar.br"]),
    (&["tar-bzip2"], &["*.tar.bz2", "*.tbz2", "*.tbz", "*.tb2"]),
    (&["tar-compressed"], TAR_COMPRESSED_GLOBS),
    (&["tar-gzip"], &["*.tar.gz", "*.tgz"]),
    (&["tar-lz4"], &["*.tar.lz4"]),
    (&["tar-lzip"], &["*.tar.lz", "*.tlz"]),
    (&["tar-lzma"], &["*.tar.lzma", "*.tar.lzm"]),
    (&["tar-pixz"], &["*.tar.pxz", "*.tpxz"]),
    (&["tar-xz"], &["*.tar.xz", "*.txz", "*.tpxz", "*.tar.pxz"]),
    (&["tar-z"], &["*.tar.Z", "*.taz"]),
    (&["tar-zstd"], &["*.tar.zst", "*.tzst"]),
    (&["taskpaper"], &["*.taskpaper"]),
    (&["tcl"], &["*.tcl"]),
    (&["tex"], &["*.tex", "*.ltx", "*.cls", "*.sty", "*.bib", "*.dtx", "*.ins"]),
    (&["texinfo"], &["*.texi"]),
    (&["textile"], &["*.textile"]),
    (&["tf"], &[
        "*.tf", "*.tf.json", "*.tfvars", "*.tfvars.json",
        "*.terraformrc", "terraform.rc", "*.tfrc", "*.terraform.lock.hcl",
    ]),
    (&["thrift"], &["*.thrift"]),
    (&["toml"], &["*.toml", "Cargo.lock"]),
    (&["ts", "typescript"], &["*.ts", "*.tsx", "*.cts", "*.mts"]),
    (&["twig"], &["*.twig"]),
    (&["txt"], &["*.txt"]),
    (&["typoscript"], &["*.typoscript", "*.ts"]),
    (&["typst"], &["*.typ"]),
    (&["usd"], &["*.usd", "*.usda", "*.usdc"]),
    (&["v"], &["*.v", "*.vsh"]),
    (&["vala"], &["*.vala"]),
    (&["vb"], &["*.vb"]),
    (&["vcl"], &["*.vcl"]),
    (&["verilog"], &["*.v", "*.vh", "*.sv", "*.svh"]),
    (&["vhdl"], &["*.vhd", "*.vhdl"]),
    (&["vim"], &[
        "*.vim", ".vimrc", ".gvimrc", "vimrc", "gvimrc", "_vimrc", "_gvimrc",
    ]),
    (&["vimscript"], &[
        "*.vim", ".vimrc", ".gvimrc", "vimrc", "gvimrc", "_vimrc", "_gvimrc",
    ]),
    (&["vue"], &["*.vue"]),
    (&["webidl"], &["*.idl", "*.webidl", "*.widl"]),
    (&["wgsl"], &["*.wgsl"]),
    (&["wiki"], &["*.mediawiki", "*.wiki"]),
    (&["xml"], &[
        "*.xml", "*.xml.dist", "*.dtd", "*.xsl", "*.xslt", "*.xsd", "*.xjb",
        "*.rng", "*.sch", "*.xhtml",
    ]),
    (&["xz"], &["*.xz"]),
    (&["yacc"], &["*.y"]),
    (&["yaml"], &["*.yaml", "*.yml"]),
    (&["yang"], &["*.yang"]),
    (&["z"], &["*.Z"]),
    (&["zig"], &["*.zig"]),
    (&["zip"], &["*.zip"]),
    (&["zsh"], &[
        ".zshenv", "zshenv",
        ".zlogin", "zlogin",
        ".zlogout", "zlogout",
        ".zprofile", "zprofile",
        ".zshrc", "zshrc",
        "*.zsh",
    ]),
    (&["zstd"], &["*.zst", "*.zstd"]),
];

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::DEFAULT_TYPES;

    fn globs(name: &str) -> BTreeSet<&'static str> {
        DEFAULT_TYPES
            .iter()
            .find(|(names, _)| names.contains(&name))
            .map(|(_, globs)| globs.iter().copied().collect())
            .unwrap_or_else(|| panic!("missing default type {name:?}"))
    }

    fn union(names: &[&str]) -> BTreeSet<&'static str> {
        names.iter().flat_map(|name| globs(name)).collect()
    }

    #[test]
    fn default_types_are_sorted() {
        let mut names = DEFAULT_TYPES.iter().map(|(aliases, _)| aliases[0]);
        let Some(mut previous_name) = names.next() else {
            return;
        };
        for name in names {
            assert!(
                name > previous_name,
                r#""{}" should be sorted before "{}" in `DEFAULT_TYPES`"#,
                name,
                previous_name
            );
            previous_name = name;
        }
    }

    #[test]
    fn archive_and_compression_types() {
        macro_rules! assert_globs {
            ($name:literal, $($glob:literal),+ $(,)?) => {
                assert_eq!(
                    globs($name),
                    [$($glob),+].into_iter().collect(),
                    "default type {:?}",
                    $name,
                );
            };
        }

        assert_globs!("7z", "*.7z");
        assert_globs!("brotli", "*.br");
        assert_globs!("bzip2", "*.bz2");
        assert_globs!("gzip", "*.gz");
        assert_globs!("lz4", "*.lz4");
        assert_globs!("lzma", "*.lzma");
        assert_globs!("rar", "*.rar");
        assert_globs!("tar", "*.tar");
        assert_globs!("tar-brotli", "*.tar.br");
        assert_globs!("tar-bzip2", "*.tar.bz2", "*.tbz2", "*.tbz", "*.tb2");
        assert_globs!("tar-gzip", "*.tar.gz", "*.tgz");
        assert_globs!("tar-lz4", "*.tar.lz4");
        assert_globs!("tar-lzip", "*.tar.lz", "*.tlz");
        assert_globs!("tar-lzma", "*.tar.lzma", "*.tar.lzm");
        assert_globs!("tar-pixz", "*.tar.pxz", "*.tpxz");
        assert_globs!("tar-xz", "*.tar.xz", "*.txz", "*.tpxz", "*.tar.pxz",);
        assert_globs!("tar-z", "*.tar.Z", "*.taz");
        assert_globs!("tar-zstd", "*.tar.zst", "*.tzst");
        assert_globs!("xz", "*.xz");
        assert_globs!("z", "*.Z");
        assert_globs!("zip", "*.zip");
        assert_globs!("zstd", "*.zst", "*.zstd");

        assert_eq!(
            globs("tar-compressed"),
            union(&[
                "tar-brotli",
                "tar-bzip2",
                "tar-gzip",
                "tar-lz4",
                "tar-lzip",
                "tar-lzma",
                "tar-pixz",
                "tar-xz",
                "tar-z",
                "tar-zstd",
            ]),
        );
        assert_eq!(
            globs("just-archive"),
            union(&["tar-compressed", "tar", "zip", "7z", "rar"]),
        );
        assert_eq!(
            globs("just-compressed"),
            union(&[
                "brotli", "bzip2", "gzip", "lz4", "lzma", "xz", "z", "zstd",
            ]),
        );
        let all = union(&["just-compressed", "just-archive"]);
        assert_eq!(globs("archive"), all);
        assert_eq!(globs("compressed"), all);
    }
}
