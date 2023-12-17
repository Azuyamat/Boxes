use crate::error::Error;
use crate::minecraft::jars;

#[test]
fn get_jar_returns_expected_jar() -> Result<(), Error> {
    let jars = jars::load()?;
    let paper_jar = jars.get_jar("paper").unwrap();
    assert_eq!(paper_jar.name, "Paper");
    Ok(())
}

#[test]
fn get_versions_returns_expected_versions() -> Result<(), Error> {
    let jars = jars::load()?;
    let paper_jar = jars.get_jar("paper").unwrap();
    let paper_versions = paper_jar.get_versions()?;
    assert!(!paper_versions.is_empty());
    Ok(())
}

#[test]
fn get_builds_returns_expected_builds() -> Result<(), Error> {
    let jars = jars::load()?;
    let paper_jar = jars.get_jar("paper").unwrap();
    let paper_versions = paper_jar.get_versions()?;
    let paper_latest = paper_versions.first().unwrap();
    let paper_builds = paper_jar.get_builds(paper_latest)?;
    assert!(!paper_builds.is_empty());
    Ok(())
}

#[test]
fn get_jar_returns_none_for_non_existent_jar() -> Result<(), Error> {
    let jars = jars::load()?;
    let non_existent_jar = jars.get_jar("non_existent");
    assert!(non_existent_jar.is_none());
    Ok(())
}
