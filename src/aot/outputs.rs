use std::path::Path;

pub fn input_stem(input: &str, fallback: &str) -> String {
    let input_path = Path::new(input);
    input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(fallback)
        .to_string()
}

pub fn output_asm_path(input: &str, custom_path: Option<&str>) -> String {
    if let Some(path) = custom_path {
        return path.to_string();
    }
    let stem = input_stem(input, "out");
    format!("target/lamina/{}.s", stem)
}

pub fn output_bin_path(input: &str) -> String {
    let stem = input_stem(input, "a");
    if cfg!(target_os = "windows") {
        format!("target/lamina/{}.exe", stem)
    } else {
        format!("target/lamina/{}", stem)
    }
}

pub fn output_ir_path(input: &str, custom_path: Option<&str>) -> String {
    if let Some(path) = custom_path {
        return path.to_string();
    }
    let stem = input_stem(input, "out");
    format!("target/lamina/{}.lamina", stem)
}
