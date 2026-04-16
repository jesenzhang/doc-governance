use anyhow::Result;

use crate::cli::TargetAgent;

pub fn render_target(path: &str, target: TargetAgent) -> Result<()> {
    println!("render target {:?} at {} (stub)", target, path);
    Ok(())
}
