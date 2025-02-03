// Generated by gir (https://github.com/gtk-rs/gir @ 5223ce91b97a)
// from ../.. (@ 0de48dc24430+)
// from ../../gir-files (@ 6cd7b656acd6)
// DO NOT EDIT

#![cfg(unix)]

use phosh_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["libphosh-0.45"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{name} {err}").into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut cmd = Command::new(exe);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }

    Ok(String::from_utf8(out.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("PhoshDBusScreenshotProxyClass", Layout {size: size_of::<PhoshDBusScreenshotProxyClass>(), alignment: align_of::<PhoshDBusScreenshotProxyClass>()}),
    ("PhoshDBusScreenshotSkeletonClass", Layout {size: size_of::<PhoshDBusScreenshotSkeletonClass>(), alignment: align_of::<PhoshDBusScreenshotSkeletonClass>()}),
    ("PhoshLayerSurface", Layout {size: size_of::<PhoshLayerSurface>(), alignment: align_of::<PhoshLayerSurface>()}),
    ("PhoshLayerSurfaceClass", Layout {size: size_of::<PhoshLayerSurfaceClass>(), alignment: align_of::<PhoshLayerSurfaceClass>()}),
    ("PhoshLockscreen", Layout {size: size_of::<PhoshLockscreen>(), alignment: align_of::<PhoshLockscreen>()}),
    ("PhoshLockscreenClass", Layout {size: size_of::<PhoshLockscreenClass>(), alignment: align_of::<PhoshLockscreenClass>()}),
    ("PhoshLockscreenManagerClass", Layout {size: size_of::<PhoshLockscreenManagerClass>(), alignment: align_of::<PhoshLockscreenManagerClass>()}),
    ("PhoshLockscreenPage", Layout {size: size_of::<PhoshLockscreenPage>(), alignment: align_of::<PhoshLockscreenPage>()}),
    ("PhoshQuickSetting", Layout {size: size_of::<PhoshQuickSetting>(), alignment: align_of::<PhoshQuickSetting>()}),
    ("PhoshQuickSettingClass", Layout {size: size_of::<PhoshQuickSettingClass>(), alignment: align_of::<PhoshQuickSettingClass>()}),
    ("PhoshScreenshotManagerClass", Layout {size: size_of::<PhoshScreenshotManagerClass>(), alignment: align_of::<PhoshScreenshotManagerClass>()}),
    ("PhoshShell", Layout {size: size_of::<PhoshShell>(), alignment: align_of::<PhoshShell>()}),
    ("PhoshShellClass", Layout {size: size_of::<PhoshShellClass>(), alignment: align_of::<PhoshShellClass>()}),
    ("PhoshStatusIcon", Layout {size: size_of::<PhoshStatusIcon>(), alignment: align_of::<PhoshStatusIcon>()}),
    ("PhoshStatusIconClass", Layout {size: size_of::<PhoshStatusIconClass>(), alignment: align_of::<PhoshStatusIconClass>()}),
    ("PhoshStatusPage", Layout {size: size_of::<PhoshStatusPage>(), alignment: align_of::<PhoshStatusPage>()}),
    ("PhoshStatusPageClass", Layout {size: size_of::<PhoshStatusPageClass>(), alignment: align_of::<PhoshStatusPageClass>()}),
    ("PhoshWallClock", Layout {size: size_of::<PhoshWallClock>(), alignment: align_of::<PhoshWallClock>()}),
    ("PhoshWallClockClass", Layout {size: size_of::<PhoshWallClockClass>(), alignment: align_of::<PhoshWallClockClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) PHOSH_LOCKSCREEN_PAGE_EXTRA", "1"),
    ("(gint) PHOSH_LOCKSCREEN_PAGE_INFO", "0"),
    ("(gint) PHOSH_LOCKSCREEN_PAGE_UNLOCK", "2"),
];


