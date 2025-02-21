use crate::compile_source::TARGETS;
use anyhow::{Context, Result, anyhow};
use std::path::Path;
use std::process::Command;

use super::EfiBinary;

pub fn compile_into(core_path: &Path, is_release: bool) -> Result<Vec<EfiBinary>> {
    let mut args = vec!["build", "-q"];
    for x in TARGETS {
        args.push("--target");
        args.push(x.triple);
    }

    if is_release {
        args.push("--release");
    }

    let mut command = Command::new("cargo")
        .args(&args)
        .current_dir(core_path)
        .spawn()
        .with_context(|| "Couldn't run cargo command")?;

    let result = command.wait()?;
    if !result.success() {
        return Err(anyhow!("Cargo failed to compile project: {result}"));
    }

    let profile = if is_release { "release" } else { "debug" };

    let binaries = TARGETS.map(|target| {
        // core/target/x86_64-unknown-uefi/debug/benstoy.efi
        let path = core_path
            .join("target")
            .join(target.triple)
            .join(profile)
            .join("benstoy.efi");

        EfiBinary { target, path }
    });

    Ok(binaries.to_vec())
}
