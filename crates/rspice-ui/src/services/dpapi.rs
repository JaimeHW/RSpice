//! Windows DPAPI at-rest protection shared by the credential stores.
//!
//! Both the offline license key and the cloud session record are secrets that
//! must never reach disk as plaintext on Windows. DPAPI binds the ciphertext
//! to the signed-in OS user, which matches how those files are scoped
//! (`%APPDATA%\rspice\...`). Unix builds instead rely on `0o600` files
//! published through the owner-only durable-file path, so this module is
//! Windows-only by design rather than by omission.

#![cfg(windows)]

/// Encrypts `plaintext` for the current OS user via `CryptProtectData`.
pub(crate) fn protect(plaintext: &[u8]) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData,
    };

    let input_len = u32::try_from(plaintext.len())
        .map_err(|_| std::io::Error::other("secret is too large for DPAPI"))?;
    let input = CRYPT_INTEGER_BLOB {
        cbData: input_len,
        pbData: plaintext.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB::default();
    let succeeded = unsafe {
        CryptProtectData(
            &input,
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(std::io::Error::last_os_error());
    }
    copy_and_free_blob(output)
}

/// Decrypts ciphertext produced by [`protect`] for the same OS user.
pub(crate) fn unprotect(ciphertext: &[u8]) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData,
    };

    let input_len = u32::try_from(ciphertext.len())
        .map_err(|_| std::io::Error::other("encrypted secret is too large for DPAPI"))?;
    let input = CRYPT_INTEGER_BLOB {
        cbData: input_len,
        pbData: ciphertext.as_ptr().cast_mut(),
    };
    let mut output = CRYPT_INTEGER_BLOB::default();
    let succeeded = unsafe {
        CryptUnprotectData(
            &input,
            std::ptr::null_mut(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return Err(std::io::Error::last_os_error());
    }
    copy_and_free_blob(output)
}

fn copy_and_free_blob(
    blob: windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB,
) -> std::io::Result<Vec<u8>> {
    use windows_sys::Win32::Foundation::LocalFree;

    struct LocalBlob(*mut u8);
    impl Drop for LocalBlob {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    LocalFree(self.0.cast());
                }
            }
        }
    }

    let allocation = LocalBlob(blob.pbData);
    if blob.cbData > 0 && allocation.0.is_null() {
        return Err(std::io::Error::other(
            "DPAPI returned a null output allocation",
        ));
    }
    let bytes = if blob.cbData == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(allocation.0, blob.cbData as usize) }
    };
    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protect_roundtrips_for_the_current_user() {
        let plaintext = b"cloud session record";
        let ciphertext = protect(plaintext).expect("DPAPI protect");
        assert_ne!(ciphertext.as_slice(), plaintext.as_slice());
        let recovered = unprotect(&ciphertext).expect("DPAPI unprotect");
        assert_eq!(recovered.as_slice(), plaintext.as_slice());
    }

    #[test]
    fn empty_input_roundtrips() {
        let ciphertext = protect(&[]).expect("DPAPI protect");
        let recovered = unprotect(&ciphertext).expect("DPAPI unprotect");
        assert!(recovered.is_empty());
    }
}
