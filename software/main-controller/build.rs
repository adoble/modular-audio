fn main() {
    // Force rebuild - including the complication of the slint file - 
    // if the sline file has changed. 
    println!("cargo:rerun-if-changed=ui/appwindow.slint");

    slint_build::compile("ui/appwindow.slint").unwrap(); 
}
