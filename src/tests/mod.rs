#[cfg(test)]
mod tests {
    use std::fs;
    use crate::error::Error;
    use crate::minecraft::jars;

    #[test]
    fn create_paper_server() -> Result<(), Error> {
        create_server("paper", "paper_test")
    }

    #[test]
    fn create_purpur_server() -> Result<(), Error> {
        create_server("purpur", "purpur_test")
    }

    //TODO: Make sure velocity and waterfall exit properly

    #[test]
    fn create_velocity_server() -> Result<(), Error> {
        create_server("velocity", "velocity_test")
    }

    #[test]
    fn create_waterfall_server() -> Result<(), Error> {
        create_server("waterfall", "waterfall_test")
    }

    fn create_server(jar_name: &str, server_name: &str) -> Result<(), Error> {
        let jar_info = jars::load()?;
        let jar = jar_info.get_jar(jar_name).unwrap();

        let current_dir = std::env::current_dir()?;
        let server_location = current_dir.join("servers").join(server_name);
        fs::create_dir_all(&server_location)?;

        let version = jar.get_versions()?;
        let version = version.first().unwrap().to_string();
        let build = jar.get_latest_build(&version).unwrap().to_string();
        let server = jar.download(&version, &build, server_name, &server_location)?;

        assert_eq!(server.server_name, server_name);

        server.run(true)?;

        let plugins_dir = server_location.join("plugins");

        assert!(plugins_dir.exists());

        server.delete();

        assert!(!server_location.exists());

        Ok(())
    }
}