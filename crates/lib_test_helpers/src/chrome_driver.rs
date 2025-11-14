use std::process::{Child, Command, Stdio};
use std::{thread, time::Duration};

pub fn chrome() -> Child {
    let child = Command::new("chromedriver")
        .arg("--port=9515")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start chromedriver");

    println!("✅ ChromeDriver started on port 9515");
    
    thread::sleep(Duration::from_millis(300));
    child
}
