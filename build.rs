use std::{env, fs, path::PathBuf};

fn main() {
    // Get the OUT_DIR environment variable
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Setup the build directory paths
    let source_dir = PathBuf::from("resources");
    let gresource_xml_file = source_dir.join("resources.gresource.xml");
    let target_dir = out_dir.join("resources.gresource");
    
    // Create the directory if it does not exist
    fs::create_dir_all(&out_dir).unwrap();
    
    // Compile and build the resources
    glib_build_tools::compile_resources(
        &[source_dir.to_str().unwrap()],
        gresource_xml_file.to_str().unwrap(),
        target_dir.to_str().unwrap(),
    );
}
