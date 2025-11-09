use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("wasm32") {
        let raylib_lib_dir = "external/raylib/web/lib";

        println!("cargo:rustc-link-lib=static=raylib");
        println!("cargo:rustc-link-search=native={}", raylib_lib_dir);

        println!("cargo:rustc-link-arg=-sMODULARIZE=1");
        println!("cargo:rustc-link-arg=-sEXPORT_NAME=createModule");
        println!("cargo:rustc-link-arg=-sENVIRONMENT=web");
        println!("cargo:rustc-link-arg=-sUSE_GLFW=3");
        println!("cargo:rustc-link-arg=-sASYNCIFY");
    } else {
        let raylib_lib_dir = "external/raylib/amd64/lib";

        println!("cargo:rustc-link-lib=static=raylib");
        println!("cargo:rustc-link-search=native={}", raylib_lib_dir);

        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-lib=dylib=GL");
            println!("cargo:rustc-link-lib=dylib=m");
            println!("cargo:rustc-link-lib=dylib=pthread");
            println!("cargo:rustc-link-lib=dylib=dl");
            println!("cargo:rustc-link-lib=dylib=rt");
        }

        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }

        #[cfg(target_os = "windows")]
        {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=opengl32");
        }
    }

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/wrappers");
}
