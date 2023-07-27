use {
    std::{env, io, process::Command},
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_TARGET_FAMILY") == Some("windows".into()) {
        WindowsResource::new()
            .set_icon("assets/ico.ico")
            .compile()?;

        let out_dir = env::var("OUT_DIR").unwrap();

        let exe_path =
            format!("C:\\Users\\user\\Desktop\\Code\\driver_v2\\target\\release\\driver_v2");

        let status = Command::new(
            "C:\\Program Files (x86)\\Windows Kits\\10\\bin\\10.0.22000.0\\x64\\mt.exe",
        )
        .arg("-manifest")
        .arg("C:\\Users\\user\\Desktop\\Code\\driver_v2\\app.manifest")
        .arg(&format!("-outputresource:{}.exe;1", exe_path))
        .current_dir(out_dir)
        .status()?;

        if !status.success() {
            panic!("Failed to attach the application manifest");
        }
    }
    Ok(())
}
