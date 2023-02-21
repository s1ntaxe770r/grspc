use tonic_build::compile_protos;

fn main () -> Result<(), Box<dyn std::error::Error>> {
  compile_protos("../protos/hello.proto")?;
  Ok(())
}
