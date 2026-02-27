mod outputs;

pub use outputs::{output_asm_path, output_bin_path, output_ir_path};

use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

pub fn ensure_parent_dir(path: &str) -> Result<(), std::io::Error> {
    let parent = Path::new(path).parent();
    if let Some(dir) = parent {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

pub fn link_assembly(asm_path: &str, bin_path: &str) -> Result<(), String> {
    let status = ProcessCommand::new("cc")
        .arg(asm_path)
        .arg("-o")
        .arg(bin_path)
        .status()
        .map_err(|e| format!("Failed to run system linker (cc): {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Lamina link failed with status {}. Assembly was saved at {}",
            status, asm_path
        ))
    }
}

pub fn executable_invocation_path(bin_path: &str) -> String {
    let path = Path::new(bin_path);
    if path.is_absolute() || bin_path.contains('/') || cfg!(target_os = "windows") {
        bin_path.to_string()
    } else {
        format!("./{}", bin_path)
    }
}
