use std::{
    io::Write,
    process::{Output, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::util::{Dir, TestCommand, sort_lines};

/// Pipe small test inputs while retaining error statuses and raw output bytes.
fn pipe_output(cmd: &mut TestCommand, input: &[u8]) -> Output {
    let mut child = cmd
        .cmd()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    if let Err(err) = stdin.write_all(input) {
        assert_eq!(std::io::ErrorKind::BrokenPipe, err.kind());
    }
    drop(stdin);
    child.wait_with_output().unwrap()
}

/// Keep stdin open without sending data, so an accidental read cannot pass.
fn output_without_stdin(cmd: &mut TestCommand) -> Output {
    let mut child = cmd
        .cmd()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let start = Instant::now();
    while child.try_wait().unwrap().is_none() {
        if start.elapsed() > Duration::from_secs(5) {
            child.kill().unwrap();
            let output = child.wait_with_output().unwrap();
            panic!("command waited for stdin: {output:?}");
        }
        thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}

rgtest!(input_order, |dir: Dir, mut cmd: TestCommand| {
    for file in ["a", "b", "c", "d"] {
        dir.create(file, "needle\n");
    }
    dir.create("lines", "a\r\n");
    dir.create_bytes("nuls", b"d\0");
    dir.create("more", "a\nc");
    cmd.args([
        "-j1",
        "-H",
        "--in",
        "lines",
        "needle",
        "b",
        "--in0=nuls",
        "--in",
        "more",
        "a",
    ]);
    eqnice!(
        "a:needle\nb:needle\nd:needle\na:needle\nc:needle\na:needle\n",
        cmd.stdout()
    );
});

rgtest!(input_pattern_sources, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n-\n");
    dir.create("paths", "a\n");
    dir.create("patterns", "needle\n");
    for args in [
        vec!["--in", "paths", "needle"],
        vec!["--in", "paths", "-e", "needle"],
        vec!["--in", "paths", "-f", "patterns"],
    ] {
        eqnice!("needle\n", dir.command().args(args).stdout());
    }
    let output = dir.command().args(["--in", "paths"]).raw_output();
    assert_eq!(Some(2), output.status.code());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("requires at least one pattern")
    );
    eqnice!("-\n", dir.command().args(["--in", "-", "-"]).pipe(b"a\n"));
});

rgtest!(input_filters_and_directories, |dir: Dir, _: TestCommand| {
    dir.create(".hidden", "needle\n");
    dir.create("ignored.txt", "needle\n");
    dir.create("plain.txt", "needle\n");
    dir.create(".ignore", "ignored.txt\ntree/ignored.rs\n");
    dir.create_dir("tree");
    dir.create("tree/one.rs", "needle\n");
    dir.create("tree/two.txt", "needle\n");
    dir.create("tree/.hidden.rs", "needle\n");
    dir.create("tree/ignored.rs", "needle\n");
    dir.create("paths", ".hidden\nignored.txt\nplain.txt\ntree\n");
    for threads in ["1", "2"] {
        let mut cmd = dir.command();
        cmd.args([
            "-j", threads, "-H", "-trust", "-g", "!*.txt", "--in", "paths",
            "needle",
        ]);
        eqnice!(
            ".hidden:needle\nignored.txt:needle\nplain.txt:needle\ntree/.hidden.rs:needle\ntree/one.rs:needle\n",
            sort_lines(&cmd.stdout())
        );
        let mut cmd = dir.command();
        cmd.args([
            "-j", threads, "-trust", "-g", "!*.txt", "--in", "paths",
            "--files",
        ]);
        eqnice!(
            ".hidden\nignored.txt\nplain.txt\ntree/.hidden.rs\ntree/one.rs\n",
            sort_lines(&cmd.stdout())
        );
    }
});

rgtest!(input_cwd_and_literal_names, |dir: Dir, _: TestCommand| {
    let files = [" a ", "-file", "#file", "glob[1]"];
    for file in files {
        dir.create(file, "needle\n");
    }
    dir.create_dir("lists");
    dir.create("lists/paths", &files.join("\n"));
    for threads in ["1", "2"] {
        let mut cmd = dir.command();
        cmd.args(["-j", threads, "-H", "--in", "lists/paths", "needle"]);
        let stdout = cmd.stdout();
        let mut lines: Vec<_> = stdout.lines().collect();
        lines.sort();
        assert_eq!(
            vec![
                " a :needle",
                "#file:needle",
                "-file:needle",
                "glob[1]:needle"
            ],
            lines
        );
    }
});

rgtest!(input_empty, |dir: Dir, _: TestCommand| {
    dir.create("empty", "");
    dir.create("a", "needle\n");
    for flag in ["--in", "--in0"] {
        for threads in ["1", "2"] {
            for mode in ["needle", "--files"] {
                for source in ["empty", "-"] {
                    let mut cmd = dir.command();
                    cmd.args(["-j", threads, flag, source, mode]);
                    let output = pipe_output(&mut cmd, b"");
                    assert_eq!(Some(1), output.status.code());
                    assert!(output.stdout.is_empty());
                    assert!(output.stderr.is_empty());
                }
                let mut cmd = dir.command();
                cmd.args(["-j", threads, flag, "empty", mode]);
                let output = pipe_output(&mut cmd, b"needle\n");
                assert_eq!(Some(1), output.status.code());
                assert!(output.stdout.is_empty());
                assert!(output.stderr.is_empty());
            }
        }
        eqnice!(
            "needle\n",
            dir.command().args([flag, "empty", "needle", "a"]).stdout()
        );
    }
});

rgtest!(input_invalid_records, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n");
    for (flag, contents, diagnostic) in [
        ("--in", &b"\n"[..], "record 1: empty path"),
        ("--in", &b"\r\n"[..], "record 1: empty path"),
        ("--in", &b"a\n\n"[..], "record 2: empty path"),
        ("--in0", &b"\0"[..], "record 1: empty path"),
        ("--in0", &b"a\0\0"[..], "record 2: empty path"),
        ("--in", &b"a\ninvalid\0path"[..], "use --in0"),
    ] {
        dir.create_bytes("paths", contents);
        let output =
            dir.command().args([flag, "paths", "needle"]).raw_output();
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(&format!("paths: {flag}:")), "{stderr}");
        assert!(stderr.contains(diagnostic), "{stderr}");
        let mut cmd = dir.command();
        cmd.args([flag, "-", "needle"]);
        let output = pipe_output(&mut cmd, contents);
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains(diagnostic));
    }
});

rgtest!(input_stdin_lists, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n");
    dir.create("b", "needle\n");
    for (flag, input) in [("--in", &b"b\n"[..]), ("--in0", &b"b\0"[..])] {
        for threads in ["1", "2"] {
            let mut cmd = dir.command();
            cmd.args(["-j", threads, "-H", "needle", "a", flag, "-"]);
            eqnice!("a:needle\nb:needle\n", sort_lines(&cmd.pipe(input)));
            let mut cmd = dir.command();
            cmd.args(["-j", threads, "--files", flag, "-"]);
            eqnice!("b\n", cmd.pipe(input));
        }
    }
});

rgtest!(input_stdin_contents_and_patterns, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n");
    dir.create("paths", "a\n");
    dir.create("dash", "-\n");
    dir.create_bytes("dash0", b"-\0");
    dir.create("-", "needle\n");
    dir.create("literal", "./-\n");
    eqnice!(
        "needle\n",
        dir.command().args(["--in", "paths", "-f", "-"]).pipe(b"needle\n")
    );
    for (flag, list) in [("--in", "dash"), ("--in0", "dash0")] {
        eqnice!(
            "needle\n",
            dir.command().args([flag, list, "needle"]).pipe(b"needle\n")
        );
        let mut cmd = dir.command();
        cmd.args([flag, list, "-f", "-"]);
        let output = pipe_output(&mut cmd, b"needle\n");
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        let mut cmd = dir.command();
        cmd.args([flag, "-", "needle"]);
        let contents = if flag == "--in" { &b"-\n"[..] } else { &b"-\0"[..] };
        let output = pipe_output(&mut cmd, contents);
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
    }
    eqnice!(
        "needle\n",
        dir.command().args(["--in", "literal", "needle"]).stdout()
    );
});

rgtest!(input_stdin_conflicts_before_reading, |dir: Dir, _: TestCommand| {
    dir.create("dash", "-\n");
    for args in [
        vec!["needle", "--in", "-", "-"],
        vec!["-f", "-", "--in0", "-"],
        vec!["needle", "--in", "-", "--in0", "-"],
        vec!["needle", "--in0", "-", "--in0", "-"],
        vec!["needle", "--in", "dash", "--in", "-"],
    ] {
        let mut cmd = dir.command();
        cmd.args(args);
        let output = output_without_stdin(&mut cmd);
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("stdin"));
    }
});

rgtest!(input_source_and_target_errors, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n");
    dir.create("paths", "a\nmissing-target\n");
    for flag in ["--in", "--in0"] {
        let output = dir
            .command()
            .args(["needle", "a", flag, "missing-list"])
            .raw_output();
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains(&format!("missing-list: {flag}:"))
        );
    }
    for threads in ["1", "2"] {
        let output = dir
            .command()
            .args(["-j", threads, "-H", "--in", "paths", "needle"])
            .raw_output();
        assert_eq!(Some(2), output.status.code());
        assert_eq!(b"a:needle\n", output.stdout.as_slice());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("missing-target")
        );
    }
});

rgtest!(input_unused_modes, |dir: Dir, mut cmd: TestCommand| {
    for args in [
        vec!["--help"],
        vec!["--version"],
        vec!["--type-list"],
        vec!["--generate", "man"],
        vec!["--generate", "complete-bash"],
        vec!["--generate", "complete-zsh"],
        vec!["--generate", "complete-fish"],
        vec!["--generate", "complete-powershell"],
    ] {
        let mut mode = dir.command();
        mode.args(args).args([
            "--in",
            "missing-list",
            "--in0",
            "missing-list",
        ]);
        let output = mode.raw_output();
        assert!(output.status.success(), "{output:?}");
        assert!(!output.stdout.is_empty());
    }
    cmd.args(["--type-list", "--in", "-", "--in0", "-"]);
    let output = output_without_stdin(&mut cmd);
    assert!(output.status.success(), "{output:?}");
});

rgtest!(input_terminators, |dir: Dir, _: TestCommand| {
    dir.create("a", "needle\n");
    dir.create("b", "needle\n");
    for (flag, contents) in [
        ("--in", &b"a\nb"[..]),
        ("--in", &b"a\r\nb\r\n"[..]),
        ("--in0", &b"a\0b"[..]),
        ("--in0", &b"a\0b\0"[..]),
    ] {
        dir.create_bytes("paths", contents);
        eqnice!(
            "a:needle\nb:needle\n",
            dir.command()
                .args(["-j1", "-H", flag, "paths", "needle"])
                .stdout()
        );
    }
    dir.create_dir("directory");
    let output =
        dir.command().args(["--in", "directory", "needle"]).raw_output();
    assert_eq!(Some(2), output.status.code());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("directory: --in:")
    );
});

#[cfg(not(unix))]
rgtest!(input_invalid_utf8, |dir: Dir, _: TestCommand| {
    for flag in ["--in", "--in0"] {
        dir.create_bytes("paths", b"invalid\xff");
        let output =
            dir.command().args([flag, "paths", "needle"]).raw_output();
        assert_eq!(Some(2), output.status.code());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("paths:"));
    }
});

rgtest!(input_config_order, |dir: Dir, mut cmd: TestCommand| {
    dir.create("a", "needle\n");
    dir.create("b", "needle\n");
    dir.create("paths", "a\n");
    dir.create("config", "--in=paths\n");
    cmd.env("RIPGREP_CONFIG_PATH", dir.path().join("config"));
    cmd.args(["-j1", "-H", "needle", "b"]);
    eqnice!("a:needle\nb:needle\n", cmd.stdout());
});

#[cfg(unix)]
rgtest!(input_unix_path_bytes, |dir: Dir, _: TestCommand| {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let files: &[&[u8]] = &[b"line\nbreak", b"tab\tname", b"carriage\r"];
    let mut list = Vec::new();
    let mut expected = Vec::new();
    for file in files {
        dir.create(OsString::from_vec(file.to_vec()), "needle\n");
        list.extend_from_slice(file);
        list.push(0);
        expected.extend_from_slice(file);
        expected.push(0);
    }
    let source = OsString::from("paths");
    dir.create_bytes(&source, &list);
    let output = dir
        .command()
        .args(["-j1", "--files", "-0", "--in0"])
        .arg(&source)
        .output();
    assert_eq!(expected, output.stdout);
    dir.create_bytes(&source, b"tab\tname\ncarriage\r");
    let output = dir
        .command()
        .args(["-j1", "-H", "--in"])
        .arg(&source)
        .arg("needle")
        .output();
    assert_eq!(
        &b"tab\tname:needle\ncarriage\r:needle\n"[..],
        output.stdout.as_slice()
    );
});

// macOS filesystems reject non-UTF-8 names; the conversion itself is tested
// independently of the filesystem in flags::hiargs::tests.
#[cfg(all(unix, not(target_os = "macos")))]
rgtest!(input_unix_non_utf8_files, |dir: Dir, mut cmd: TestCommand| {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    dir.create(OsString::from_vec(b"file\xff".to_vec()), "needle\n");
    let source = OsString::from_vec(b"paths\xfe".to_vec());
    dir.create_bytes(&source, b"file\xff\0");
    cmd.args(["--in0"]).arg(source).arg("needle");
    let output = cmd.output();
    assert_eq!(&b"needle\n"[..], output.stdout.as_slice());
});
