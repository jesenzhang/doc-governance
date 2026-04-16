use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn init_project(path: &str) -> Result<()> {
    let root = Path::new(path);

    fs::create_dir_all(root.join("openspec/specs"))?;
    fs::create_dir_all(root.join("openspec/changes"))?;
    fs::create_dir_all(root.join("governance"))?;
    fs::create_dir_all(root.join("agents"))?;

    write_if_missing(
        &root.join("governance/DOC_SYSTEM.md"),
        "# DOC_SYSTEM\n\nDefine documentation layers, source-of-truth order, and allowed artifact types.\n",
    )?;
    write_if_missing(
        &root.join("governance/DOC_ROUTING.md"),
        "# DOC_ROUTING\n\nMap change types to required document updates.\n",
    )?;
    write_if_missing(
        &root.join("governance/CHANGE_POLICY.md"),
        "# CHANGE_POLICY\n\nDefine when docs/specs/ADR/plans must be updated.\n",
    )?;
    write_if_missing(
        &root.join("openspec/config.yaml"),
        "context:\n  - governance/DOC_SYSTEM.md\n  - governance/DOC_ROUTING.md\n  - governance/CHANGE_POLICY.md\n",
    )?;

    println!("initialized governance/OpenSpec skeleton at {}", root.display());
    Ok(())
}

pub fn new_change(path: &str, name: &str) -> Result<()> {
    let root = Path::new(path);
    let change_root = root.join("openspec/changes").join(name);

    fs::create_dir_all(change_root.join("specs/core"))?;
    fs::write(change_root.join("proposal.md"), format!("# Proposal: {}\n\n## Why\n\n## What changes\n\n## Impact\n", name))?;
    fs::write(change_root.join("design.md"), "# Design\n\n## Overview\n\n## Architecture\n\n## Risks\n")?;
    fs::write(change_root.join("tasks.md"), "# Tasks\n\n- [ ] Define scope\n- [ ] Update specs\n- [ ] Implement\n- [ ] Validate consistency\n")?;
    fs::write(change_root.join("specs/core/spec.md"), "# Spec Delta\n\n## ADDED Requirements\n\n## MODIFIED Requirements\n\n## REMOVED Requirements\n")?;

    println!("created change workspace: {}", change_root.display());
    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}
