use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use notch::jars::manager::JarManager;
use notch::servers::runner::Runner;
use crate::error::Error;

#[test]
fn get_jar_returns_expected_jar() -> Result<(), Error> {
    let jars = JarManager::load()?;
    let paper_jar = jars.get_jar("paper".to_string()).unwrap();
    assert_eq!(paper_jar.name, "Paper");
    Ok(())
}

#[test]
fn get_versions_returns_expected_versions() -> Result<(), Error> {
    let jars = JarManager::load()?;
    let paper_jar = jars.get_jar("paper".to_string()).unwrap();
    let paper_versions = paper_jar.get_versions()?;
    assert!(!paper_versions.is_empty());
    Ok(())
}

#[test]
fn get_builds_returns_expected_builds() -> Result<(), Error> {
    let jars = JarManager::load()?;
    let paper_jar = jars.get_jar("paper".to_string()).unwrap();
    let paper_versions = paper_jar.get_versions()?;
    let paper_latest = paper_versions.first().unwrap();
    let paper_builds = paper_jar.get_builds(paper_latest.clone())?;
    assert!(!paper_builds.is_empty());
    Ok(())
}

#[test]
fn get_jar_returns_none_for_non_existent_jar() -> Result<(), Error> {
    let jars = JarManager::load()?;
    let non_existent_jar = jars.get_jar("non_existent".to_string());
    assert!(non_existent_jar.is_err());
    Ok(())
}

#[test]
fn start_server_works() -> Result<(), Error> {
    let name = "test".to_string();
    let config = crate::config::Config::load()?;
    let server = config
        .get_server(&name)
        .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
    let mut runner = Runner::new(&server, vec![], vec![]);

    let mut child = runner.start()?;
    while child.stdout.is_none() {
        println!("📝 Can't find STDOUT! Waiting...");
        thread::sleep(Duration::from_secs(1));
    }

    if let Some(ref mut stdout) = child.stdout {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let text = line.unwrap();
            if text.contains("EULA") {
                println!("📝 Accepting EULA...");
                server.accept_eula()?;
                println!("📝 Accepted EULA!");
            }
        }
    } else {
        println!("📝 Can't find STDOUT!");
        child.kill()?;
    }

    child.wait()?;

    Ok(())
}