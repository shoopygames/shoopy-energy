#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("energy.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    // No-op on Linux/WSL
}