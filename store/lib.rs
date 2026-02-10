pub mod verify;

pub use verify::{
    parse_manifest, verify_package, verify_signature, PackageManifest, PackagePermissions,
    StoreVerifyError,
};
