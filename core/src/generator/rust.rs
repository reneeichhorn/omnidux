use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use cargo_toml_builder::prelude::*;

pub struct RustProjectBuilder {
  name: String,
  destination: String,
}

impl RustProjectBuilder {
  // Creates a new builder to build rust projects
  pub fn new(name: String) -> Self {
    Self {
      name: name,
      destination: "".to_string(),
    }
  }

  pub fn set_destination(mut self, destination: String) -> Self {
    self.destination = destination;
    self
  }

  #[allow(unused_must_use)]
  pub fn build(self) -> Result<(), Box<dyn std::error::Error>> {
    // Build toml configuration for cargo.
    let toml = CargoToml::builder()
      .name(&self.name)
      .author("Autogenerated <>")
      .version("1.0.0")
      .build()?;
    
    // Prepare project folder 
    let destination = format!("{d}/{n}-main-lib", d = self.destination, n = self.name);
    let destination_path = Path::new(&destination);
    fs::remove_dir_all(destination_path); 
    fs::create_dir_all(destination_path)?;
    fs::create_dir_all(format!("{d}/src", d = destination))?;

    // Insert cargo
    let mut file = File::create(format!("{d}/Cargo.toml", d = destination))?;
    file.write_all(&toml.to_string().into_bytes())?;
    Ok(())
  }
}
