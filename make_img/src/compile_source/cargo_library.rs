use std::path::Path;

use anyhow::Result;
use cargo::{
    GlobalContext,
    core::{
        Verbosity, Workspace,
        compiler::{CompileKind, CompileMode, CompileTarget},
    },
    ops::{CompileOptions, compile},
    util::interning::InternedString,
};

use crate::compile_source::TARGETS;

use super::EfiBinary;

pub fn compile_into(core_path: &Path, is_release: bool) -> Result<Vec<EfiBinary>> {
    let mut global_context = GlobalContext::default()?;
    global_context.shell().set_verbosity(Verbosity::Quiet);
    global_context.reload_rooted_at(core_path)?; // Load config files at workspace root of `core`

    let mut compile_options = CompileOptions::new(&global_context, CompileMode::Build)?;
    if is_release {
        compile_options.build_config.requested_profile = InternedString::new("release");
    }

    // Set targets (the cargo library can compile for multiple targets at once)
    compile_options.build_config.requested_kinds = TARGETS
        .map(|target| Ok(CompileKind::Target(CompileTarget::new(target.triple)?)))
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    Ok(compile(
        &Workspace::new(&core_path.join("Cargo.toml"), &global_context)?,
        &compile_options,
    )?
    .binaries
    .into_iter()
    .map(|binary| EfiBinary {
        path: binary.path,
        target: match binary.unit.kind {
            CompileKind::Host => unreachable!("Requested targets were explicitely set."),
            CompileKind::Target(target) => target.short_name().into(),
        },
    })
    .collect())
}
