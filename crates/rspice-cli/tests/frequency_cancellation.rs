//! Frequency-domain CLI analyses must propagate the process abort into core
//! execution and must not publish a result artifact after cancellation.
//! Accepted model completion retains the solved prefix and succeeds.

mod common;

use common::test_dir;

use std::process::Command;

#[test]
fn accepted_frequency_finish_publishes_the_completed_prefix() {
    let directory = test_dir("frequency_finish");
    let model = directory.path().join("finish.va");
    std::fs::write(
        &model,
        r#"
module finish_model(p,n);
inout p,n; electrical p,n;
real count;
analog begin
    @(initial_step) count=1;
    @(final_step) count=count+1;
    if (!analysis("static") && count==1) $finish(1);
    I(p,n)<+count*1e-3*V(p,n);
    I(p,n)<+white_noise(count*1e-18,"model_noise");
end
endmodule"#,
    )
    .expect("write finish model");
    for (name, analysis) in [
        ("ac", ".ac lin 13 10 1meg"),
        ("noise", ".noise V(out) V1 lin 13 10 1meg"),
    ] {
        let deck = directory.path().join(format!("{name}.cir"));
        let artifact = directory.path().join(format!("{name}.csv"));
        std::fs::write(&deck, format!(
            "* Accepted frequency finish\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nX1 out 0 finish_model\n.va \"{}\" finish_model\n{analysis}\n.end\n",
            model.to_string_lossy().replace('\\', "/"),
        )).expect("write frequency deck");
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "run",
                deck.to_str().unwrap(),
                "--output",
                artifact.to_str().unwrap(),
                "--format",
                "csv",
            ])
            .output()
            .expect("run default-stack frequency fixture");
        assert!(
            output.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let csv = std::fs::read_to_string(&artifact).expect("accepted finish publishes CSV");
        let rows = csv
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>();
        assert_eq!(
            rows.len(),
            2,
            "{name} must publish its header and one solved row: {csv}"
        );
        let frequency = rows[1].split(',').next().unwrap().parse::<f64>().unwrap();
        assert_eq!(frequency, 10.0);
    }
}

#[test]
fn noise_timeout_is_typed_prompt_and_does_not_publish_an_artifact() {
    let directory = test_dir("noise");
    let deck = directory.path().join("long_noise.sp");
    let artifact = directory.path().join("noise.csv");
    std::fs::write(
        &deck,
        "* cancellable frequency-domain analysis\n\
         V1 in 0 DC 0 AC 1\n\
         R1 in out 1k\n\
         R2 out 0 2k\n\
         .noise V(out) V1 lin 1500000 1 1meg\n\
         .end\n",
    )
    .expect("write noise fixture");

    let started = std::time::Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--timeout",
            "0.25",
            "-o",
            artifact.to_str().expect("UTF-8 artifact path"),
            "-f",
            "csv",
        ])
        .output()
        .expect("run rspice");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(124),
        "frequency timeout must retain the typed timeout exit code\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("timed out") || stderr.contains("Timeout"),
        "timeout diagnostic must remain explicit:\n{stderr}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "frequency cancellation must stop promptly"
    );
    assert!(
        !artifact.exists(),
        "a cancelled analysis must not publish {}",
        artifact.display()
    );
}
