//! Packer and verifier for `.rspicepack` archives.
//!
//! The tool is a thin shell over the library: it reads files, hands bytes to
//! the same functions a client runs, and prints the verdict. Argument parsing
//! is deliberately hand-rolled — a signing tool should not carry a parser
//! dependency it does not need.
//!
//! ```text
//! rspice-pack keygen --out <secret-file>
//! rspice-pack build <dir> --template <template.json> --key <secret-file> -o <pack>
//! rspice-pack verify <pack> --pubkey <hex>
//! ```
//!
//! `keygen` writes 32 raw secret bytes and prints the public key as lowercase
//! hexadecimal on standard output. It refuses to overwrite an existing file.

use std::{
    collections::BTreeMap,
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    process::ExitCode,
};

use rspice_pack::{
    Limits, ManifestTemplate, Pack, SIGNING_KEY_BYTES, build_pack_from_dir, encode_hex,
    generate_signing_key, sha256_hex, signing_key, verifying_key_from_hex,
};

const USAGE: &str = "\
usage:
  rspice-pack keygen --out <secret-file>
  rspice-pack build <dir> --template <template.json> --key <secret-file> -o <pack>
  rspice-pack verify <pack> --pubkey <hex>";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let Some((command, rest)) = arguments.split_first() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if command == "--help" || command == "-h" {
        println!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    let parsed = match Arguments::parse(rest) {
        Ok(parsed) => parsed,
        Err(reason) => {
            eprintln!("{reason}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let outcome = match command.as_str() {
        "keygen" => keygen(&parsed),
        "build" => build(&parsed),
        "verify" => verify(&parsed),
        other => {
            eprintln!("unknown command {other:?}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("rspice-pack: {reason}");
            ExitCode::FAILURE
        }
    }
}

struct Arguments {
    positional: Vec<String>,
    options: BTreeMap<String, String>,
}

impl Arguments {
    fn parse(tokens: &[String]) -> Result<Self, String> {
        let mut positional = Vec::new();
        let mut options = BTreeMap::new();
        let mut cursor = tokens.iter();
        while let Some(token) = cursor.next() {
            if token.len() > 1 && token.starts_with('-') {
                let Some(value) = cursor.next() else {
                    return Err(format!("{token} needs a value"));
                };
                if options.insert(token.clone(), value.clone()).is_some() {
                    return Err(format!("{token} was given more than once"));
                }
            } else {
                positional.push(token.clone());
            }
        }
        Ok(Self {
            positional,
            options,
        })
    }

    fn option(&self, name: &str) -> Result<&str, String> {
        self.options
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| format!("{name} is required"))
    }

    fn only_positional(&self, description: &str) -> Result<&str, String> {
        match self.positional.as_slice() {
            [single] => Ok(single),
            _ => Err(format!("expected exactly one {description}")),
        }
    }
}

fn keygen(arguments: &Arguments) -> Result<(), String> {
    let destination = PathBuf::from(arguments.option("--out")?);
    let key = generate_signing_key().map_err(|error| error.to_string())?;
    write_secret(&destination, &key.to_bytes())?;
    println!("{}", encode_hex(key.verifying_key().as_bytes()));
    Ok(())
}

fn build(arguments: &Arguments) -> Result<(), String> {
    let directory = PathBuf::from(arguments.only_positional("source directory")?);
    let template = read_template(Path::new(arguments.option("--template")?))?;
    let key = signing_key(&read_secret(Path::new(arguments.option("--key")?))?);
    let destination = PathBuf::from(arguments.option("-o")?);
    let archive =
        build_pack_from_dir(&directory, template, &key).map_err(|error| error.to_string())?;
    let mut file = fs::File::create_new(&destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    file.write_all(&archive)
        .map_err(|error| format!("cannot write {}: {error}", destination.display()))?;
    println!(
        "{} {} bytes sha256={}",
        destination.display(),
        archive.len(),
        sha256_hex(&archive)
    );
    Ok(())
}

fn verify(arguments: &Arguments) -> Result<(), String> {
    let source = PathBuf::from(arguments.only_positional("pack archive")?);
    let key =
        verifying_key_from_hex(arguments.option("--pubkey")?).map_err(|error| error.to_string())?;
    let archive = read_file(&source)?;
    let pack =
        Pack::verify(&archive, &key, &Limits::default()).map_err(|error| error.to_string())?;
    println!(
        "{} {} verified: {} files, {} parts",
        pack.manifest.pack.id,
        pack.manifest.pack.version,
        pack.manifest.files.len(),
        pack.manifest.parts.len()
    );
    Ok(())
}

fn read_template(path: &Path) -> Result<ManifestTemplate, String> {
    let bytes = read_file(path)?;
    // Templates are JSON so the tool shares the crate's only serialization
    // dependency; a TOML front end would add one for no gain.
    serde_json::from_slice(&bytes).map_err(|error| {
        format!(
            "{} is not a valid JSON manifest template: {error}",
            path.display()
        )
    })
}

fn read_secret(path: &Path) -> Result<[u8; SIGNING_KEY_BYTES], String> {
    let bytes = read_file(path)?;
    bytes.as_slice().try_into().map_err(|_| {
        format!(
            "{} must hold exactly {SIGNING_KEY_BYTES} raw secret bytes, found {}",
            path.display(),
            bytes.len()
        )
    })
}

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))
}

fn write_secret(path: &Path, secret: &[u8; SIGNING_KEY_BYTES]) -> Result<(), String> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|error| format!("cannot create {}: {error}", path.display()))?;
    file.write_all(secret)
        .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    file.sync_all()
        .map_err(|error| format!("cannot flush {}: {error}", path.display()))
}
