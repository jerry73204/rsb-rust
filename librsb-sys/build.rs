use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::{
    borrow::Cow,
    env,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    #[cfg(not(feature = "doc-only"))]
    {
        #[cfg(target_os = "linux")]
        {
            #[cfg(feature = "codegen")]
            codegen::codegen()?;

            link()?;
        }

        #[cfg(not(target_os = "linux"))]
        {
            anyhow::bail!("unsupported OS");
        }
    }

    Ok(())
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use bindgen::EnumVariation;

    #[allow(dead_code)]
    pub fn codegen() -> Result<()> {
        const BINDINGS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs");

        let include_dir =
            probe_include_dir().ok_or_else(|| anyhow!("unable to find include dir"))?;
        let header_files = ["rsb.h", "blas_sparse.h", "rsb_types.h", "rsb-config.h"];

        // Tell cargo to invalidate the built crate whenever the wrapper changes

        let mut build = bindgen::Builder::default();

        for file in header_files {
            let path = format!("{}/{}", include_dir.display(), file);
            println!("cargo:rerun-if-changed={}", path);
            build = build.header(path);
        }

        let build = build
            .derive_partialeq(true)
            .derive_eq(true)
            .derive_hash(true)
            .allowlist_type("blas_.*")
            .allowlist_type("rsb_.*")
            .allowlist_type("BLAS_.*")
            .allowlist_type("RSB_.*")
            .allowlist_function("BLAS_.*")
            .allowlist_function("blas_.*")
            .allowlist_function("rsb_.*")
            .allowlist_var("BLAS_.*")
            .allowlist_var("RSB_.*")
            .default_enum_style(EnumVariation::Rust {
                non_exhaustive: false,
            })
            .newtype_enum("blas_handle_type")
            .new_type_alias("rsb_err_t")
            .new_type_alias("rsb_type_t")
            .new_type_alias("rsb_trans_t")
            .new_type_alias("blas_sparse_matrix");

        let bindings = build
            .generate()
            .map_err(|_| anyhow!("Unable to generate bindings"))?;

        bindings
            .write_to_file(BINDINGS_FILE)
            .map_err(|_| anyhow!("Couldn't write bindings!"))?;

        Ok(())
    }
}

#[allow(dead_code)]
fn link() -> Result<()> {
    let library_dir = probe_library_dir().ok_or_else(|| anyhow!("unable to find library dir"))?;
    println!("cargo:rustc-link-search={}", library_dir.display());
    println!("cargo:rustc-link-lib=dylib=rsb");
    Ok(())
}

use probe::*;
mod probe {
    #![allow(dead_code)]
    use super::*;

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

    pub fn probe_include_dir() -> Option<Cow<'static, Path>> {
        let path: Option<Cow<'_, Path>> = LIBRSB_INCLUDE
            .as_ref()
            .map(|path| Cow::Borrowed(path.as_ref()));

        let path = path.or_else(|| -> Option<Cow<'_, Path>> {
            let path = LIBRSB_PATH.as_ref()?;
            Some(Cow::Owned(path.join("include")))
        });

        path.or_else(|| Some(Cow::Borrowed(Path::new("/usr/include"))))
    }

    pub fn probe_library_dir() -> Option<Cow<'static, Path>> {
        let path: Option<Cow<'_, Path>> = LIBRSB_LIBRARY
            .as_ref()
            .map(|path| Cow::Borrowed(path.as_ref()));

        let path = path.or_else(|| -> Option<Cow<'_, Path>> {
            let path = LIBRSB_PATH.as_ref()?;
            Some(Cow::Owned(path.join("lib")))
        });

        path.or_else(|| Some(Cow::Borrowed(Path::new("/usr/lib"))))
    }
}
