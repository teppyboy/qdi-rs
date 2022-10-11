use std::{fs, path::PathBuf, process::Command};

fn install_vcredist(steamworks_redist: &PathBuf) {
    let vcredist = steamworks_redist.join("vcredist");
    let paths = fs::read_dir(vcredist).unwrap();
    
    for path in paths {
        let redist = path.unwrap().path();
        println!("Scanning: {}", redist.display());
        if !redist.is_dir() {
            continue;
        }
        let x86 = redist.join("vcredist_x86.exe");
        if x86.is_file() {
            println!("Found: {}", x86.display());
            Command::new(x86)
                .args(["/quiet", "/norestart"])
                .output()
                .expect("Failed to start process");
        }
        let x64 = redist.join("vcredist_x64.exe");
        if x64.is_file() {
            println!("Found: {}", x64.display());
            Command::new(x64)
                .args(["/quiet", "/norestart"])
                .output()
                .expect("Failed to start process");
        }
    }
}

fn main() {
    println!("{}", format!("QuickDependenciesInstaller v{} by tretrauit", env!("CARGO_PKG_VERSION")));
    let steamworks_redist = PathBuf::from("./Steamworks Shared/_CommonRedist");
    if !steamworks_redist.is_dir() {
        panic!("Steamworks Shared/_CommonRedist doesn't exist.");
    }
    install_vcredist(&steamworks_redist);
    println!("Installation finished.")
}
