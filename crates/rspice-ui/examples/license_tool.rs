//! Development license tooling (spec: docs/license-key-spec.md).
//!
//! Production issuance belongs to the platform backend's cold-key flow;
//! this exists to mint the development signer (key id 0x01) and signed
//! test fixtures.
//!
//! ```text
//! cargo run -p rspice-ui --example license_tool -- gen
//! cargo run -p rspice-ui --example license_tool -- issue \
//!     --secret <hex64> --key-id 1 --name "Jaime Whitfield" --tier 1 \
//!     --issued-days 20615 --expires-days 20979 --features 7 \
//!     [--license-id 9f3a58c177d20b4e] [--seats 0]
//! ```

use ed25519_dalek::{Signer, SigningKey};
use rspice_ui::services::license::{
    LicensePayload, SIGNING_DOMAIN, crockford_encode, date_from_unix_days, group5,
    parse_and_verify,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("gen") => generate(),
        Some("issue") => issue(&args[1..]),
        _ => {
            eprintln!("usage: license_tool gen | issue --secret <hex64> [options]");
            std::process::exit(2);
        }
    }
}

fn generate() {
    let signing = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying = signing.verifying_key();
    println!("secret (keep OUT of the repository):");
    println!("  {}", hex(&signing.to_bytes()));
    println!("public:");
    println!("  {}", hex(verifying.as_bytes()));
    println!("public as a Rust array:");
    println!("  {}", rust_array(verifying.as_bytes()));
}

fn issue(args: &[String]) {
    let get = |name: &str| -> Option<String> {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let secret_hex = get("--secret").unwrap_or_else(|| die("--secret <hex64> is required"));
    let secret: [u8; 32] = unhex(&secret_hex)
        .and_then(|v| v.try_into().ok())
        .unwrap_or_else(|| die("--secret must be 64 hex chars"));
    let signing = SigningKey::from_bytes(&secret);

    let key_id: u8 = parse_or(get("--key-id"), 1);
    let tier: u8 = parse_or(get("--tier"), 1);
    let seats: u16 = parse_or(get("--seats"), 0);
    let issued_days: u32 = parse_or(get("--issued-days"), 20_615);
    let expires_days: u32 = parse_or(get("--expires-days"), issued_days + 365);
    let features: u32 = parse_or(get("--features"), 7);
    let license_id: u64 = match get("--license-id") {
        Some(text) => u64::from_str_radix(&text.replace('-', ""), 16)
            .unwrap_or_else(|_| die("--license-id must be hex")),
        None => {
            // Random id from the OS, formatted for support/denylist use.
            let mut bytes = [0u8; 8];
            getrandom(&mut bytes);
            u64::from_be_bytes(bytes)
        }
    };

    let payload = LicensePayload {
        version: 1,
        key_id,
        license_id,
        tier,
        seats,
        issued_days,
        expires_days,
        features,
        licensee: get("--name"),
        email_hash: None,
    };
    let payload_bytes = payload.to_bytes();

    let mut message = Vec::with_capacity(SIGNING_DOMAIN.len() + payload_bytes.len());
    message.extend_from_slice(SIGNING_DOMAIN);
    message.extend_from_slice(&payload_bytes);
    let signature = signing.sign(&message);

    let key = format!(
        "RSPICE-K1.{}.{}",
        crockford_encode(&payload_bytes),
        crockford_encode(&signature.to_bytes())
    );
    println!("key (canonical):");
    println!("  {key}");
    println!("key (grouped for email):");
    let (p, s) = key
        .strip_prefix("RSPICE-K1.")
        .and_then(|r| r.split_once('.'))
        .expect("just formatted");
    println!("  RSPICE-K1.{}.{}", group5(p), group5(s));
    println!(
        "grant: tier {tier} · updates until {} · license id {license_id:016x}",
        date_from_unix_days(expires_days)
    );

    // Self-check through the app's verifier so the fixture can never be
    // committed broken. (Verifies structure+signature; key-id must be in
    // the binary's table for the app itself to accept it.)
    match parse_and_verify(&key) {
        Ok(info) => println!("self-verify against compiled-in keys: OK ({})", info.tier),
        Err(error) => println!(
            "self-verify against compiled-in keys: {:?} (expected for a fresh key id not yet in VERIFYING_KEYS)",
            error
        ),
    }
}

fn parse_or<T: std::str::FromStr>(value: Option<String>, default: T) -> T {
    value
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Option<Vec<u8>> {
    let text = text.trim();
    if text.len() % 2 != 0 {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).ok())
        .collect()
}

fn rust_array(bytes: &[u8]) -> String {
    let body = bytes
        .iter()
        .map(|b| format!("0x{b:02x}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{body}]")
}

fn getrandom(out: &mut [u8]) {
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(out);
}

fn die(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(2)
}
