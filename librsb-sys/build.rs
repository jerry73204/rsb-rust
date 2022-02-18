use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::{
    borrow::Cow,
    env,
    path::{Path, PathBuf},
};

static LIBRSB_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
    println!("cargo:rerun-if-env-changed=LIBRSB_PATH");
    let path = env::var_os("LIBRSB_PATH")?;
    Some(PathBuf::from(path))
});
static LIBRSB_LIBRARY: Lazy<Option<PathBuf>> = Lazy::new(|| {
    println!("cargo:rerun-if-env-changed=LIBRSB_LIBRARY");
    let path = env::var_os("LIBRSB_LIBRARY")?;
    Some(PathBuf::from(path))
});
static LIBRSB_INCLUDE: Lazy<Option<PathBuf>> = Lazy::new(|| {
    println!("cargo:rerun-if-env-changed=LIBRSB_INCLUDE");
    let path = env::var_os("LIBRSB_INCLUDE")?;
    Some(PathBuf::from(path))
});

fn main() -> Result<()> {
    #[cfg(not(feature = "doc-only"))]
    {
        #[cfg(target_os = "linux")]
        {
            #[cfg(feature = "codegen")]
            codegen()?;

            link()?;
        }

        #[cfg(not(target_os = "linux"))]
        {
            bail!("unsupported OS");
        }
    }

    Ok(())
}

#[cfg(feature = "codegen")]
fn codegen() -> Result<()> {
    const BINDINGS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs");

    let include_dir = probe_include_dir().ok_or_else(|| anyhow!("unable to find include dir"))?;
    let header_files = ["rsb.h", "blas_sparse.h", "rsb_types.h", "rsb-config.h"];

    // // Tell cargo to invalidate the built crate whenever the wrapper changes

    let mut build = bindgen::Builder::default();

    for file in header_files {
        let path = format!("{}/{}", include_dir.display(), file);
        println!("cargo:rerun-if-changed={}", path);
        build = build.header(path);
    }

    let bindings = build
        .generate()
        .map_err(|_| anyhow!("Unable to generate bindings"))?;

    bindings
        .write_to_file(BINDINGS_FILE)
        .map_err(|_| anyhow!("Couldn't write bindings!"))?;

    Ok(())
}

fn link() -> Result<()> {
    let library_dir = probe_library_dir().ok_or_else(|| anyhow!("unable to find library dir"))?;
    println!("cargo:rustc-link-search={}", library_dir.display());
    println!("cargo:rustc-link-lib=dylib=rsb");
    Ok(())
}

// fn probe_library_linux() -> Result<Library> {
//     let include = probe_include_dir().ok_or_else(|| anyhow!("unable to find include dir"))?;
//     let library = probe_library_dir().ok_or_else(|| anyhow!("unable to find library dir"))?;

//     Ok(Library { include, library })
// }

fn probe_include_dir() -> Option<Cow<'static, Path>> {
    let path: Option<Cow<'_, Path>> = LIBRSB_INCLUDE
        .as_ref()
        .map(|path| Cow::Borrowed(path.as_ref()));

    let path = path.or_else(|| -> Option<Cow<'_, Path>> {
        let path = LIBRSB_PATH.as_ref()?;
        Some(Cow::Owned(path.join("include")))
    });

    path.or_else(|| Some(Cow::Borrowed(Path::new("/usr/include"))))
}

fn probe_library_dir() -> Option<Cow<'static, Path>> {
    let path: Option<Cow<'_, Path>> = LIBRSB_LIBRARY
        .as_ref()
        .map(|path| Cow::Borrowed(path.as_ref()));

    let path = path.or_else(|| -> Option<Cow<'_, Path>> {
        let path = LIBRSB_PATH.as_ref()?;
        Some(Cow::Owned(path.join("lib")))
    });

    path.or_else(|| Some(Cow::Borrowed(Path::new("/usr/lib"))))
}

struct Library {
    pub include: Cow<'static, Path>,
    pub library: Cow<'static, Path>,
}
