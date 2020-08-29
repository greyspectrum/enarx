use std::process::Command;

fn main() {
    let output = Command::new("curl")
        .arg("-v")
        .arg("https://api.trustedservices.intel.com/sgx/certification/v1/pckcrl?ca={processor}")
        .arg("2>&1")
        .arg("|")
        .arg("awk")
        .arg("-F")
        .arg(r#""SGX-PCK-CRL-Issuer-Chain: ""#)
        .arg(r#"'{print $2}'"#)
        .arg("|")
        .arg("sed")
        .output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }
}
