use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    let mut buf = [0u8; 11];

    let child1 = Command::new("curl")
        .arg("-v")
        .arg("https://api.trustedservices.intel.com/sgx/certification/v1/pckcrl?ca={processor}")
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    let child2 = Command::new("tac")
        .stdin(child1.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    let child3 = Command::new("tac")
        .stdin(child2.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    let child4 = Command::new("awk")
        .arg("-F")
        .arg(r#""SGX-PCK-CRL-Issuer-Chain: ""#)
        .arg(r#"'{print $2}'"#)
        .stdin(child3.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    let child5 = Command::new("sed")
        .arg("-e")
        .arg(":a")
        .arg("-e")
        .arg(r#"'s@%@\\x@g;/./,$!d;/^\n*$/{$d;N;};/\n$/ba'"#)
        .stdin(child4.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    let child6 = Command::new("xargs")
        .arg("-0")
        .arg("printf")
        .arg(r#""%b""#)
        .stdin(child5.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail");

    child6.stdout.unwrap().read(&mut buf).expect("fail");

    println!("output: {:?}", String::from_utf8(buf.to_vec()).unwrap());
}
