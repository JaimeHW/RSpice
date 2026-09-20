//! Offline publisher for organization-managed RSpice drawing-sheet packages.
//!
//! The GUI never accepts private publisher material. This native-only tool
//! performs the narrowly scoped signing ceremony and immediately verifies the
//! emitted artifact against the exact package contract used by the importer.

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    #[cfg(unix)]
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::{Read, Write as _};
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;
    use std::sync::atomic::{AtomicU64, Ordering};

    use base64::{Engine as _, engine::general_purpose::STANDARD};
    #[cfg(test)]
    use clap::CommandFactory as _;
    use clap::{Args, Parser, Subcommand, ValueEnum};
    use rspice_design_model::sheet_package::{
        DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding,
        drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
        publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
    };
    use zeroize::{Zeroize as _, Zeroizing};

    const MAX_KEY_FILE_BYTES: usize = 4 * 1024;
    const PUBLIC_KEY_HEADER: &str = "RSPICE-ED25519-PUBLIC-KEY-V1";
    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[cfg(windows)]
    const WINDOWS_SEALED_KEY_PREFIX: &[u8] = b"RSPICE-SHEET-PUBLISHER-DPAPI\0\x01";
    #[cfg(windows)]
    const WINDOWS_DPAPI_ENTROPY: &[u8] = b"RSPICE\0SHEET-PUBLISHER-KEY\0V1";

    #[derive(Debug, Parser)]
    #[command(
        name = "rspice-sheet-publisher",
        version,
        about = "Sign and verify organization-managed RSpice drawing-sheet packages",
        long_about = "Offline publisher for organization-managed RSpice drawing-sheet packages. Private key material is read only from a protected file and is never accepted as a command-line value. Output paths must not already exist."
    )]
    struct Cli {
        #[command(subcommand)]
        command: Command,
    }

    #[derive(Debug, Subcommand)]
    enum Command {
        /// Promote an unsigned export to organization scope and sign it.
        Sign(SignArgs),
        /// Verify a signed package against an explicitly pinned public key.
        Verify(VerifyArgs),
        /// Derive the public key to provision into the RSpice trust store.
        PublicKey(PublicKeyArgs),
        /// Seal a plaintext seed with Windows DPAPI for subsequent signing.
        #[cfg(windows)]
        SealKey(SealKeyArgs),
    }

    #[derive(Debug, Args)]
    struct SignArgs {
        /// Unsigned canonical or human-review package exported by RSpice.
        #[arg(long, value_name = "FILE")]
        input: PathBuf,
        /// New signed package path. It must not already exist.
        #[arg(long, value_name = "FILE")]
        output: PathBuf,
        /// Protected 32-byte Ed25519 seed file.
        #[arg(long, value_name = "FILE")]
        private_key_file: PathBuf,
        /// Public key file used to pin and cross-check the private key.
        #[arg(long, value_name = "FILE")]
        public_key_file: PathBuf,
        /// Lowercase organization publisher identity provisioned in RSpice.
        #[arg(long)]
        publisher_id: String,
        /// Lowercase key identity provisioned in RSpice.
        #[arg(long)]
        key_id: String,
        /// SHA-256 contract digest reported for the reviewed unsigned export.
        #[arg(long)]
        expected_input_digest: String,
        /// Explicitly change every exported preset to organization scope.
        #[arg(long)]
        promote_to_organization: bool,
        /// Signed output representation.
        #[arg(long, value_enum, default_value_t = OutputFormat::Canonical)]
        format: OutputFormat,
    }

    #[derive(Debug, Args)]
    struct VerifyArgs {
        /// Signed package to verify.
        #[arg(long, value_name = "FILE")]
        input: PathBuf,
        /// Pinned Ed25519 public key file.
        #[arg(long, value_name = "FILE")]
        public_key_file: PathBuf,
        /// Expected publisher identity; payload identity is never trusted implicitly.
        #[arg(long)]
        publisher_id: String,
        /// Expected signing-key identity.
        #[arg(long)]
        key_id: String,
    }

    #[derive(Debug, Args)]
    struct PublicKeyArgs {
        /// Protected 32-byte Ed25519 seed file.
        #[arg(long, value_name = "FILE")]
        private_key_file: PathBuf,
        /// New public key path. It must not already exist.
        #[arg(long, value_name = "FILE")]
        output: PathBuf,
    }

    #[cfg(windows)]
    #[derive(Debug, Args)]
    struct SealKeyArgs {
        /// Plain 32-byte or 64-hex-character seed file, or '-' for stdin.
        #[arg(long, value_name = "FILE")]
        input_key_file: PathBuf,
        /// New DPAPI-sealed private key path. It must not already exist.
        #[arg(long, value_name = "FILE")]
        output: PathBuf,
        /// New public key path. It must not already exist.
        #[arg(long, value_name = "FILE")]
        public_key_output: PathBuf,
    }

    #[derive(Debug, Clone, Copy, Default, ValueEnum)]
    enum OutputFormat {
        #[default]
        Canonical,
        HumanReview,
    }

    impl From<OutputFormat> for DrawingSheetPackageEncoding {
        fn from(value: OutputFormat) -> Self {
            match value {
                OutputFormat::Canonical => Self::CanonicalSchema1,
                OutputFormat::HumanReview => Self::HumanReviewJson,
            }
        }
    }

    pub(super) fn main() -> ExitCode {
        match run(Cli::parse_from(std::env::args_os())) {
            Ok(message) => {
                println!("{message}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        }
    }

    fn run(cli: Cli) -> Result<String, String> {
        match cli.command {
            Command::Sign(args) => sign(args),
            Command::Verify(args) => verify(args),
            Command::PublicKey(args) => write_public_key(args),
            #[cfg(windows)]
            Command::SealKey(args) => seal_key(args),
        }
    }

    fn sign(args: SignArgs) -> Result<String, String> {
        reject_aliases(
            &args.output,
            [&args.input, &args.private_key_file, &args.public_key_file],
        )?;
        let source = read_utf8_file_bounded(
            &args.input,
            DRAWING_SHEET_PACKAGE_MAX_BYTES,
            "input package",
        )?;
        let expected_input_digest = encode_hex(&parse_hex_key(
            args.expected_input_digest.trim(),
            "expected input digest",
        )?);
        let inspected = inspect_drawing_sheet_package(&source)?;
        if inspected.is_signed {
            return Err("the reviewed input package is already signed".to_owned());
        }
        if inspected.source_digest_sha256 != expected_input_digest {
            return Err(format!(
                "input package digest {} does not match reviewed digest {}",
                inspected.source_digest_sha256, expected_input_digest
            ));
        }
        let private_seed = Zeroizing::new(load_private_seed(&args.private_key_file)?);
        let pinned_public_key = load_public_key(&args.public_key_file)?;
        let derived_public_key = drawing_sheet_publisher_public_key(&private_seed);
        if derived_public_key != pinned_public_key {
            return Err(format!(
                "private key '{}' does not match pinned public key '{}'",
                args.private_key_file.display(),
                args.public_key_file.display()
            ));
        }

        let published = publish_organization_drawing_sheet_package(
            &source,
            &private_seed,
            &args.publisher_id,
            &args.key_id,
            args.promote_to_organization,
            args.format.into(),
        )?;
        if published.verifying_key != pinned_public_key {
            return Err("publisher self-check returned an unexpected public key".to_owned());
        }
        let bytes = package_output_bytes(&published.encoded);
        publish_new_file(&args.output, &bytes, 0o644)?;

        let written = read_utf8_file_bounded(
            &args.output,
            DRAWING_SHEET_PACKAGE_MAX_BYTES + 1,
            "published package",
        )?;
        let verified = verify_published_drawing_sheet_package(&written, &pinned_public_key)?;
        require_organization_package(verified.preset_count, verified.organization_preset_count)?;
        require_identity(
            &verified.publisher_id,
            &verified.signing_key_id,
            &args.publisher_id,
            &args.key_id,
        )?;
        if verified.source_digest_sha256 != published.source_digest_sha256
            || verified.preset_count != published.preset_count
        {
            return Err("published package read-back did not match its signing receipt".to_owned());
        }

        Ok(format!(
            "published {} organization drawing-sheet preset(s) to '{}' (publisher {}/{}, digest {})",
            verified.organization_preset_count,
            args.output.display(),
            verified.publisher_id,
            verified.signing_key_id,
            verified.source_digest_sha256
        ))
    }

    fn verify(args: VerifyArgs) -> Result<String, String> {
        let source = read_utf8_file_bounded(
            &args.input,
            DRAWING_SHEET_PACKAGE_MAX_BYTES + 1,
            "signed package",
        )?;
        let public_key = load_public_key(&args.public_key_file)?;
        let verified = verify_published_drawing_sheet_package(&source, &public_key)?;
        require_organization_package(verified.preset_count, verified.organization_preset_count)?;
        require_identity(
            &verified.publisher_id,
            &verified.signing_key_id,
            &args.publisher_id,
            &args.key_id,
        )?;
        Ok(format!(
            "verified {} preset(s), including {} organization preset(s), for {}/{} (digest {})",
            verified.preset_count,
            verified.organization_preset_count,
            verified.publisher_id,
            verified.signing_key_id,
            verified.source_digest_sha256
        ))
    }

    fn write_public_key(args: PublicKeyArgs) -> Result<String, String> {
        reject_aliases(&args.output, [&args.private_key_file])?;
        let private_seed = Zeroizing::new(load_private_seed(&args.private_key_file)?);
        let public_key = drawing_sheet_publisher_public_key(&private_seed);
        publish_new_file(&args.output, &public_key_document(&public_key), 0o644)?;
        Ok(format!(
            "wrote Ed25519 public key to '{}'",
            args.output.display()
        ))
    }

    #[cfg(windows)]
    fn seal_key(args: SealKeyArgs) -> Result<String, String> {
        reject_destination_alias(&args.output, &args.public_key_output)?;
        if args.input_key_file != Path::new("-") {
            reject_aliases(&args.output, [&args.input_key_file])?;
            reject_aliases(&args.public_key_output, [&args.input_key_file])?;
        }
        ensure_new_destination(&args.output)?;
        ensure_new_destination(&args.public_key_output)?;

        let mut source = if args.input_key_file == Path::new("-") {
            read_stdin_bounded(MAX_KEY_FILE_BYTES)?
        } else {
            Zeroizing::new(read_regular_file_bounded(
                &args.input_key_file,
                MAX_KEY_FILE_BYTES,
                "plaintext private key",
            )?)
        };
        let seed = Zeroizing::new(parse_key_material(&source, "private key")?);
        source.zeroize();
        let public_key = drawing_sheet_publisher_public_key(&seed);
        let encrypted = windows_dpapi_protect(seed.as_ref())?;

        // A partial failure may leave only the non-secret public key. The
        // sealed private key is published last, so success means both exist.
        publish_new_file(
            &args.public_key_output,
            &public_key_document(&public_key),
            0o644,
        )?;
        let mut sealed = Vec::with_capacity(WINDOWS_SEALED_KEY_PREFIX.len() + encrypted.len());
        sealed.extend_from_slice(WINDOWS_SEALED_KEY_PREFIX);
        sealed.extend_from_slice(&encrypted);
        publish_new_file(&args.output, &sealed, 0o600)?;
        sealed.zeroize();

        Ok(format!(
            "sealed the private key to '{}' and wrote its public key to '{}'; the plaintext source was not deleted",
            args.output.display(),
            args.public_key_output.display()
        ))
    }

    fn require_identity(
        actual_publisher: &str,
        actual_key: &str,
        expected_publisher: &str,
        expected_key: &str,
    ) -> Result<(), String> {
        if actual_publisher != expected_publisher || actual_key != expected_key {
            return Err(format!(
                "signed identity {actual_publisher}/{actual_key} does not match expected identity {expected_publisher}/{expected_key}"
            ));
        }
        Ok(())
    }

    fn require_organization_package(
        preset_count: usize,
        organization_preset_count: usize,
    ) -> Result<(), String> {
        if preset_count == 0 || organization_preset_count != preset_count {
            return Err(format!(
                "publisher artifact must contain only organization presets; found {organization_preset_count} of {preset_count}"
            ));
        }
        Ok(())
    }

    fn package_output_bytes(encoded: &str) -> Vec<u8> {
        encoded.as_bytes().to_vec()
    }

    fn public_key_document(public_key: &[u8; 32]) -> Vec<u8> {
        format!(
            "{PUBLIC_KEY_HEADER}\nalgorithm:ed25519\nhex:{}\nbase64:{}\n",
            encode_hex(public_key),
            STANDARD.encode(public_key)
        )
        .into_bytes()
    }

    fn load_public_key(path: &Path) -> Result<[u8; 32], String> {
        let bytes = read_regular_file_bounded(path, MAX_KEY_FILE_BYTES, "public key")?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| format!("public key '{}' is not UTF-8 text", path.display()))?;
        if !text.starts_with(PUBLIC_KEY_HEADER) {
            return parse_hex_key(text.trim(), "public key");
        }
        let mut algorithm = None;
        let mut hex = None;
        let mut base64 = None;
        for line in text.lines().skip(1) {
            if line.is_empty() {
                continue;
            }
            let (name, value) = line.split_once(':').ok_or_else(|| {
                format!("public key '{}' contains a malformed field", path.display())
            })?;
            match name {
                "algorithm" if algorithm.replace(value).is_none() => {}
                "hex" if hex.replace(value).is_none() => {}
                "base64" if base64.replace(value).is_none() => {}
                _ => {
                    return Err(format!(
                        "public key '{}' contains an unknown or duplicate field '{name}'",
                        path.display()
                    ));
                }
            }
        }
        if algorithm != Some("ed25519") {
            return Err(format!(
                "public key '{}' does not declare algorithm ed25519",
                path.display()
            ));
        }
        let from_hex = parse_hex_key(
            hex.ok_or_else(|| format!("public key '{}' has no hex field", path.display()))?,
            "public key",
        )?;
        let decoded =
            STANDARD
                .decode(base64.ok_or_else(|| {
                    format!("public key '{}' has no base64 field", path.display())
                })?)
                .map_err(|error| {
                    format!(
                        "public key '{}' has invalid base64: {error}",
                        path.display()
                    )
                })?;
        let from_base64: [u8; 32] = decoded.try_into().map_err(|bytes: Vec<u8>| {
            format!(
                "public key '{}' base64 contains {} bytes; exactly 32 are required",
                path.display(),
                bytes.len()
            )
        })?;
        if from_hex != from_base64 {
            return Err(format!(
                "public key '{}' hex and base64 fields disagree",
                path.display()
            ));
        }
        Ok(from_hex)
    }

    #[cfg(unix)]
    fn load_private_seed(path: &Path) -> Result<[u8; 32], String> {
        use std::os::unix::fs::{MetadataExt as _, OpenOptionsExt as _, PermissionsExt as _};

        let named = std::fs::symlink_metadata(path).map_err(|error| {
            format!(
                "could not inspect private key '{}': {error}",
                path.display()
            )
        })?;
        if !named.file_type().is_file() || named.nlink() != 1 {
            return Err(format!(
                "private key '{}' must be one regular, non-symlink file",
                path.display()
            ));
        }
        if named.permissions().mode() & 0o077 != 0 {
            return Err(format!(
                "private key '{}' must not grant group or other permissions (use mode 0600)",
                path.display()
            ));
        }
        if named.uid() != rustix::process::geteuid().as_raw() {
            return Err(format!(
                "private key '{}' is not owned by the effective user",
                path.display()
            ));
        }
        let mut options = OpenOptions::new();
        options
            .read(true)
            .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
        let mut file = options
            .open(path)
            .map_err(|error| format!("could not open private key '{}': {error}", path.display()))?;
        let opened = file
            .metadata()
            .map_err(|error| format!("could not inspect opened private key: {error}"))?;
        if opened.dev() != named.dev()
            || opened.ino() != named.ino()
            || opened.nlink() != 1
            || opened.uid() != named.uid()
            || opened.permissions().mode() & 0o077 != 0
        {
            return Err("private key changed while its security was checked".to_owned());
        }
        let mut bytes = Zeroizing::new(read_bounded(&mut file, MAX_KEY_FILE_BYTES, "private key")?);
        let parsed = parse_key_material(&bytes, "private key");
        bytes.zeroize();
        parsed
    }

    #[cfg(windows)]
    fn load_private_seed(path: &Path) -> Result<[u8; 32], String> {
        let mut sealed = Zeroizing::new(read_regular_file_bounded(
            path,
            MAX_KEY_FILE_BYTES,
            "sealed private key",
        )?);
        let ciphertext = sealed
            .strip_prefix(WINDOWS_SEALED_KEY_PREFIX)
            .ok_or_else(|| {
                format!(
                    "private key '{}' is not an RSpice DPAPI-sealed publisher key; use the seal-key command first",
                    path.display()
                )
            })?;
        let plaintext = windows_dpapi_unprotect(ciphertext)?;
        sealed.zeroize();
        if plaintext.len() != 32 {
            return Err(format!(
                "unsealed private key contains {} bytes; exactly 32 are required",
                plaintext.len()
            ));
        }
        let mut seed = [0_u8; 32];
        seed.copy_from_slice(&plaintext);
        Ok(seed)
    }

    fn parse_key_material(bytes: &[u8], label: &str) -> Result<[u8; 32], String> {
        if bytes.len() == 32 {
            let mut key = [0_u8; 32];
            key.copy_from_slice(bytes);
            return Ok(key);
        }
        let text = std::str::from_utf8(bytes).map_err(|_| {
            format!("{label} must contain 32 raw bytes or 64 hexadecimal characters")
        })?;
        parse_hex_key(text.trim(), label)
    }

    fn parse_hex_key(value: &str, label: &str) -> Result<[u8; 32], String> {
        if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(format!(
                "{label} must contain exactly 64 hexadecimal characters"
            ));
        }
        let mut key = [0_u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            key[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
        }
        Ok(key)
    }

    fn hex_nibble(byte: u8) -> Result<u8, String> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err("key contains a non-hexadecimal character".to_owned()),
        }
    }

    fn encode_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        encoded
    }

    fn read_utf8_file_bounded(path: &Path, limit: usize, label: &str) -> Result<String, String> {
        let bytes = read_regular_file_bounded(path, limit, label)?;
        String::from_utf8(bytes)
            .map_err(|_| format!("{label} '{}' is not valid UTF-8", path.display()))
    }

    fn read_regular_file_bounded(
        path: &Path,
        limit: usize,
        label: &str,
    ) -> Result<Vec<u8>, String> {
        let named = std::fs::symlink_metadata(path)
            .map_err(|error| format!("could not inspect {label} '{}': {error}", path.display()))?;
        if !named.file_type().is_file() {
            return Err(format!(
                "{label} '{}' must be a regular, non-symlink file",
                path.display()
            ));
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

            if named.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(format!(
                    "{label} '{}' must not be a reparse point",
                    path.display()
                ));
            }
        }
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;

            options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;

            options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
        }
        let mut file = options
            .open(path)
            .map_err(|error| format!("could not open {label} '{}': {error}", path.display()))?;
        let opened = file
            .metadata()
            .map_err(|error| format!("could not inspect opened {label}: {error}"))?;
        if !opened.file_type().is_file() {
            return Err(format!(
                "{label} '{}' did not open as a regular file",
                path.display()
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;

            if opened.dev() != named.dev() || opened.ino() != named.ino() {
                return Err(format!(
                    "{label} '{}' changed while it was opened",
                    path.display()
                ));
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt as _;
            use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

            if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(format!(
                    "{label} '{}' opened as a reparse point",
                    path.display()
                ));
            }
        }
        read_bounded(&mut file, limit, label)
    }

    fn read_bounded(reader: &mut impl Read, limit: usize, label: &str) -> Result<Vec<u8>, String> {
        let bounded = u64::try_from(limit)
            .ok()
            .and_then(|limit| limit.checked_add(1))
            .ok_or_else(|| format!("{label} size limit is invalid"))?;
        let mut bytes = Vec::new();
        reader
            .take(bounded)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("could not read {label}: {error}"))?;
        if bytes.len() > limit {
            return Err(format!("{label} exceeds the {limit} byte limit"));
        }
        Ok(bytes)
    }

    #[cfg(windows)]
    fn read_stdin_bounded(limit: usize) -> Result<Zeroizing<Vec<u8>>, String> {
        let mut stdin = std::io::stdin().lock();
        read_bounded(&mut stdin, limit, "private key from stdin").map(Zeroizing::new)
    }

    fn reject_aliases<'a>(
        output: &Path,
        inputs: impl IntoIterator<Item = &'a PathBuf>,
    ) -> Result<(), String> {
        let output_identity = destination_identity(output)?;
        for input in inputs {
            let input_identity = std::fs::canonicalize(input).map_err(|error| {
                format!("could not resolve input '{}': {error}", input.display())
            })?;
            if paths_equal(&output_identity, &input_identity) {
                return Err(format!(
                    "output '{}' must be different from every input path",
                    output.display()
                ));
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn reject_destination_alias(left: &Path, right: &Path) -> Result<(), String> {
        if paths_equal(&destination_identity(left)?, &destination_identity(right)?) {
            return Err(format!(
                "output paths '{}' and '{}' must be different",
                left.display(),
                right.display()
            ));
        }
        Ok(())
    }

    fn destination_identity(path: &Path) -> Result<PathBuf, String> {
        let file_name = path
            .file_name()
            .ok_or_else(|| format!("output '{}' has no file name", path.display()))?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let parent = std::fs::canonicalize(parent).map_err(|error| {
            format!(
                "could not resolve output directory '{}': {error}",
                parent.display()
            )
        })?;
        Ok(parent.join(file_name))
    }

    #[cfg(windows)]
    fn paths_equal(left: &Path, right: &Path) -> bool {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    }

    #[cfg(not(windows))]
    fn paths_equal(left: &Path, right: &Path) -> bool {
        left == right
    }

    fn ensure_new_destination(path: &Path) -> Result<(), String> {
        match std::fs::symlink_metadata(path) {
            Ok(_) => Err(format!(
                "output '{}' already exists; publisher outputs are never overwritten",
                path.display()
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(format!(
                "could not inspect output '{}': {error}",
                path.display()
            )),
        }
    }

    fn publish_new_file(path: &Path, bytes: &[u8], _unix_mode: u32) -> Result<(), String> {
        ensure_new_destination(path)?;
        let requested_parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let file_name = path
            .file_name()
            .ok_or_else(|| format!("output '{}' has no file name", path.display()))?;
        let parent_metadata = std::fs::symlink_metadata(requested_parent).map_err(|error| {
            format!(
                "could not inspect output directory '{}': {error}",
                requested_parent.display()
            )
        })?;
        if !parent_metadata.file_type().is_dir() {
            return Err(format!(
                "output parent '{}' is not a directory",
                requested_parent.display()
            ));
        }
        let parent = std::fs::canonicalize(requested_parent).map_err(|error| {
            format!(
                "could not resolve output directory '{}': {error}",
                requested_parent.display()
            )
        })?;
        let destination = parent.join(file_name);
        ensure_new_destination(&destination)?;

        let mut last_collision = None;
        for _ in 0..128 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let temp = parent.join(format!(
                ".rspice-sheet-publisher-{}-{sequence}.tmp",
                std::process::id()
            ));
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt as _;
                options.mode(_unix_mode);
            }
            let mut file = match options.open(&temp) {
                Ok(file) => file,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    last_collision = Some(error);
                    continue;
                }
                Err(error) => {
                    return Err(format!(
                        "could not create publisher staging file in '{}': {error}",
                        parent.display()
                    ));
                }
            };
            let mut guard = TemporaryFile::new(temp.clone());
            file.write_all(bytes)
                .and_then(|()| file.sync_all())
                .map_err(|error| {
                    format!("could not durably stage '{}': {error}", path.display())
                })?;
            drop(file);

            let staging_path_remains =
                publish_staged_no_replace(&temp, &destination).map_err(|error| {
                    if error.kind() == std::io::ErrorKind::AlreadyExists {
                        format!(
                            "output '{}' was created concurrently and was not overwritten",
                            path.display()
                        )
                    } else {
                        format!("could not publish output '{}': {error}", path.display())
                    }
                })?;
            if !staging_path_remains {
                guard.disarm();
            }
            let written = read_regular_file_bounded(&destination, bytes.len(), "published output")?;
            if written != bytes {
                return Err(format!(
                    "published output '{}' failed its byte-for-byte read-back check",
                    path.display()
                ));
            }
            if staging_path_remains {
                guard.remove()?;
            }
            sync_directory(&parent)?;
            return Ok(());
        }
        Err(format!(
            "could not allocate a unique staging file in '{}': {}",
            parent.display(),
            last_collision
                .map(|error| error.to_string())
                .unwrap_or_else(|| "name collisions exhausted".to_owned())
        ))
    }

    struct TemporaryFile {
        path: PathBuf,
        removed: bool,
    }

    impl TemporaryFile {
        fn new(path: PathBuf) -> Self {
            Self {
                path,
                removed: false,
            }
        }

        fn remove(mut self) -> Result<(), String> {
            std::fs::remove_file(&self.path).map_err(|error| {
                format!(
                    "published output is valid, but staging file '{}' could not be removed: {error}",
                    self.path.display()
                )
            })?;
            self.removed = true;
            Ok(())
        }

        fn disarm(&mut self) {
            self.removed = true;
        }
    }

    #[cfg(not(windows))]
    fn publish_staged_no_replace(staged: &Path, destination: &Path) -> std::io::Result<bool> {
        std::fs::hard_link(staged, destination)?;
        Ok(true)
    }

    #[cfg(windows)]
    fn publish_staged_no_replace(staged: &Path, destination: &Path) -> std::io::Result<bool> {
        use std::os::windows::ffi::OsStrExt as _;
        use windows_sys::Win32::Storage::FileSystem::{MOVEFILE_WRITE_THROUGH, MoveFileExW};

        let staged: Vec<u16> = staged.as_os_str().encode_wide().chain(Some(0)).collect();
        let destination: Vec<u16> = destination
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let moved = unsafe {
            MoveFileExW(
                staged.as_ptr(),
                destination.as_ptr(),
                MOVEFILE_WRITE_THROUGH,
            )
        };
        if moved == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(false)
        }
    }

    impl Drop for TemporaryFile {
        fn drop(&mut self) {
            if !self.removed {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }

    #[cfg(unix)]
    fn sync_directory(path: &Path) -> Result<(), String> {
        File::open(path)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| {
                format!(
                    "could not durably synchronize output directory '{}': {error}",
                    path.display()
                )
            })
    }

    #[cfg(windows)]
    fn sync_directory(_path: &Path) -> Result<(), String> {
        // The linked file itself is flushed before publication. Opening a
        // directory for FlushFileBuffers requires backup-semantics privileges
        // and is not consistently supported by Windows filesystems.
        Ok(())
    }

    #[cfg(windows)]
    fn windows_dpapi_protect(plaintext: &[u8]) -> Result<Zeroizing<Vec<u8>>, String> {
        use windows_sys::Win32::Security::Cryptography::{
            CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData,
        };

        let input = crypt_blob(plaintext, "private key")?;
        let entropy = crypt_blob(WINDOWS_DPAPI_ENTROPY, "DPAPI entropy")?;
        let mut output = CRYPT_INTEGER_BLOB::default();
        let succeeded = unsafe {
            CryptProtectData(
                &input,
                std::ptr::null(),
                &entropy,
                std::ptr::null(),
                std::ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        if succeeded == 0 {
            return Err(format!(
                "Windows DPAPI could not seal the private key: {}",
                std::io::Error::last_os_error()
            ));
        }
        copy_and_free_dpapi_blob(output, "sealed private key")
    }

    #[cfg(windows)]
    fn windows_dpapi_unprotect(ciphertext: &[u8]) -> Result<Zeroizing<Vec<u8>>, String> {
        use windows_sys::Win32::Security::Cryptography::{
            CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptUnprotectData,
        };

        let input = crypt_blob(ciphertext, "sealed private key")?;
        let entropy = crypt_blob(WINDOWS_DPAPI_ENTROPY, "DPAPI entropy")?;
        let mut output = CRYPT_INTEGER_BLOB::default();
        let succeeded = unsafe {
            CryptUnprotectData(
                &input,
                std::ptr::null_mut(),
                &entropy,
                std::ptr::null(),
                std::ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        if succeeded == 0 {
            return Err(format!(
                "Windows DPAPI could not unseal the private key for this user: {}",
                std::io::Error::last_os_error()
            ));
        }
        copy_and_free_dpapi_blob(output, "unsealed private key")
    }

    #[cfg(windows)]
    fn crypt_blob(
        bytes: &[u8],
        label: &str,
    ) -> Result<windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB, String> {
        use windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;

        Ok(CRYPT_INTEGER_BLOB {
            cbData: u32::try_from(bytes.len())
                .map_err(|_| format!("{label} is too large for Windows DPAPI"))?,
            pbData: bytes.as_ptr().cast_mut(),
        })
    }

    #[cfg(windows)]
    fn copy_and_free_dpapi_blob(
        blob: windows_sys::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB,
        label: &str,
    ) -> Result<Zeroizing<Vec<u8>>, String> {
        use windows_sys::Win32::Foundation::LocalFree;

        struct LocalBlob {
            pointer: *mut u8,
            length: usize,
        }
        impl Drop for LocalBlob {
            fn drop(&mut self) {
                if !self.pointer.is_null() {
                    unsafe {
                        std::ptr::write_bytes(self.pointer, 0, self.length);
                        LocalFree(self.pointer.cast());
                    }
                }
            }
        }

        let allocation = LocalBlob {
            pointer: blob.pbData,
            length: blob.cbData as usize,
        };
        if allocation.pointer.is_null() && blob.cbData != 0 {
            return Err(format!("Windows DPAPI returned a null {label} buffer"));
        }
        let bytes = if blob.cbData == 0 {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(allocation.pointer.cast_const(), blob.cbData as usize)
                    .to_vec()
            }
        };
        Ok(Zeroizing::new(bytes))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct TestDirectory(PathBuf);

        impl TestDirectory {
            fn new() -> Self {
                let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
                let path = std::env::temp_dir().join(format!(
                    "rspice-sheet-publisher-test-{}-{sequence}",
                    std::process::id()
                ));
                std::fs::create_dir(&path).expect("create test directory");
                Self(path)
            }
        }

        impl Drop for TestDirectory {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        #[test]
        fn key_parser_accepts_exact_raw_and_hex_seeds() {
            assert_eq!(parse_key_material(&[0x5a; 32], "key").unwrap(), [0x5a; 32]);
            let hex = format!("{}\n", "5a".repeat(32));
            assert_eq!(
                parse_key_material(hex.as_bytes(), "key").unwrap(),
                [0x5a; 32]
            );
            assert!(parse_key_material(b"5a", "key").is_err());
        }

        #[test]
        fn atomic_publication_never_overwrites_an_existing_destination() {
            let directory = TestDirectory::new();
            let output = directory.0.join("package.json");
            publish_new_file(&output, b"first", 0o644).unwrap();
            let error = publish_new_file(&output, b"second", 0o644).unwrap_err();
            assert!(error.contains("never overwritten"));
            assert_eq!(std::fs::read(&output).unwrap(), b"first");
            assert!(std::fs::read_dir(&directory.0).unwrap().all(|entry| {
                !entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains(".tmp")
            }));
        }

        #[test]
        fn output_aliases_are_rejected_before_publication() {
            let directory = TestDirectory::new();
            let input = directory.0.join("input.json");
            std::fs::write(&input, b"source").unwrap();
            let error = reject_aliases(&input, [&input]).unwrap_err();
            assert!(error.contains("different from every input"));
        }

        #[cfg(windows)]
        #[test]
        fn two_new_output_paths_cannot_alias() {
            let directory = TestDirectory::new();
            let output = directory.0.join("output.json");
            let error = reject_destination_alias(&output, &output).unwrap_err();
            assert!(error.contains("must be different"));
        }

        #[cfg(windows)]
        #[test]
        fn windows_dpapi_sealed_keys_round_trip_only_as_ciphertext() {
            let seed = [0x73; 32];
            let encrypted = windows_dpapi_protect(&seed).unwrap();
            assert_ne!(encrypted.as_slice(), seed);
            let restored = windows_dpapi_unprotect(&encrypted).unwrap();
            assert_eq!(restored.as_slice(), seed);
        }

        #[cfg(windows)]
        #[test]
        fn windows_seal_command_provisions_matching_private_and_public_files() {
            let directory = TestDirectory::new();
            let source = directory.0.join("source.seed");
            let sealed = directory.0.join("publisher.dpapi");
            let public = directory.0.join("publisher.pub");
            std::fs::write(&source, [0x36; 32]).unwrap();

            seal_key(SealKeyArgs {
                input_key_file: source.clone(),
                output: sealed.clone(),
                public_key_output: public.clone(),
            })
            .unwrap();

            assert_eq!(std::fs::read(&source).unwrap(), [0x36; 32]);
            assert_eq!(load_private_seed(&sealed).unwrap(), [0x36; 32]);
            assert_eq!(
                load_public_key(&public).unwrap(),
                drawing_sheet_publisher_public_key(&[0x36; 32])
            );
            assert!(!std::fs::read(&sealed).unwrap().ends_with(&[0x36; 32]));
        }

        #[test]
        fn public_key_document_round_trips_through_the_loader() {
            let directory = TestDirectory::new();
            let path = directory.0.join("publisher.pub");
            let key = [0xa5; 32];
            publish_new_file(&path, &public_key_document(&key), 0o644).unwrap();
            assert_eq!(load_public_key(&path).unwrap(), key);

            let mut mismatched = String::from_utf8(public_key_document(&key)).unwrap();
            let replacement = STANDARD.encode([0xa4; 32]);
            let base64_start = mismatched.find("base64:").unwrap() + "base64:".len();
            let base64_end = mismatched[base64_start..].find('\n').unwrap() + base64_start;
            mismatched.replace_range(base64_start..base64_end, &replacement);
            let mismatch_path = directory.0.join("mismatched.pub");
            std::fs::write(&mismatch_path, mismatched).unwrap();
            assert!(
                load_public_key(&mismatch_path)
                    .unwrap_err()
                    .contains("disagree")
            );
        }

        #[cfg(unix)]
        #[test]
        fn unix_private_keys_require_owner_only_regular_single_link_files() {
            use std::os::unix::fs::{PermissionsExt as _, symlink};

            let directory = TestDirectory::new();
            let path = directory.0.join("publisher.seed");
            std::fs::write(&path, [0x4c; 32]).unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(load_private_seed(&path).unwrap_err().contains("mode 0600"));

            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
            assert_eq!(load_private_seed(&path).unwrap(), [0x4c; 32]);

            let hard_link = directory.0.join("publisher-hard-link.seed");
            std::fs::hard_link(&path, &hard_link).unwrap();
            assert!(
                load_private_seed(&path)
                    .unwrap_err()
                    .contains("one regular, non-symlink file")
            );
            std::fs::remove_file(&hard_link).unwrap();

            let symbolic_link = directory.0.join("publisher-symbolic-link.seed");
            symlink(&path, &symbolic_link).unwrap();
            assert!(
                load_private_seed(&symbolic_link)
                    .unwrap_err()
                    .contains("one regular, non-symlink file")
            );
        }

        #[test]
        fn command_line_never_defines_a_private_key_value_option() {
            let mut command = Cli::command();
            let help = command
                .find_subcommand_mut("sign")
                .expect("sign subcommand")
                .render_long_help()
                .to_string();
            assert!(!help.contains("--private-key "));
            assert!(help.contains("--private-key-file"));
            assert!(help.contains("--expected-input-digest"));
        }

        #[test]
        fn publisher_verification_requires_a_nonempty_all_organization_package() {
            assert!(require_organization_package(1, 1).is_ok());
            assert!(require_organization_package(0, 0).is_err());
            assert!(require_organization_package(2, 1).is_err());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> std::process::ExitCode {
    native::main()
}
