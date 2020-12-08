use std::{path, io, fs, env};

fn application_root_dir() -> Result<path::PathBuf, io::Error> {
    if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(path::PathBuf::from(manifest_dir));
    }

    let mut exe = fs::canonicalize(env::current_exe()?)?;

    // Modify in-place to avoid an extra copy.
    if exe.pop() {
        return Ok(exe);
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to find an application root",
    ))
}

pub fn data_file(path: &'static str) -> Result<fs::File, io::Error> {
    let app_root = application_root_dir()?;
    let data_path = app_root.join("data").join(path);
    fs::File::open(data_path)
}
