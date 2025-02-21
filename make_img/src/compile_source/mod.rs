#[cfg(not(feature = "embed_cargo"))]
mod cargo_command;
#[cfg(feature = "embed_cargo")]
mod cargo_library;

use anyhow::Result;

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Target {
    triple: &'static str,
    pub efi_boot_file: &'static str,
}

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        TARGETS
            .iter()
            .find(|target| value == target.triple)
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't find the target triple {}.\nChoose one of {}, {} or {}",
                    value, TARGETS[0].triple, TARGETS[1].triple, TARGETS[2].triple
                )
            })
            .clone()
    }
}

#[derive(Debug, Clone)]
pub struct EfiBinary {
    pub path: PathBuf,
    pub target: Target,
}

const TARGETS: [Target; 3] = [
    Target {
        triple: "x86_64-unknown-uefi",
        efi_boot_file: "BOOTX64.EFI",
    },
    Target {
        triple: "aarch64-unknown-uefi",
        efi_boot_file: "BOOTAA64.EFI",
    },
    Target {
        triple: "i686-unknown-uefi",
        efi_boot_file: "BOOTIA32.EFI",
    },
];

pub fn compile(is_release: bool) -> Result<Vec<EfiBinary>> {
    let core_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent() // $GIT_ROOT$
        .expect("$CARGO_MANIFEST_DIR is '/'")
        .join("core"); // $GIT_ROOT$/core

    #[cfg(not(feature = "embed_cargo"))]
    return cargo_command::compile_into(&core_path, is_release);

    #[cfg(feature = "embed_cargo")]
    return cargo_library::compile_into(&core_path, is_release);
}
