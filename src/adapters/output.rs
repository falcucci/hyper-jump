use crate::ports::Output;

pub struct StdoutOutput;

impl Output for StdoutOutput {
    fn write_line(&self, line: &str) -> anyhow::Result<()> {
        println!("{line}");
        Ok(())
    }
}
