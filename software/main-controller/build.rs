fn main() {
    // Force rebuild - including the complication of the slint file - 
    // if the slint file has changed. 
    //println!("cargo:rerun-if-changed=ui/appwindow.slint");
    println!("cargo:rerun-if-changed=ui");


    slint_build::compile("ui/appwindow.slint").unwrap(); 
}
