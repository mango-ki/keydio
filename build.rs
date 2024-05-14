fn main() {
    //set icon for application on build
    println!("cargo:rustc-link-arg=assets\\embed_icon.res");
}