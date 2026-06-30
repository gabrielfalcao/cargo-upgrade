pub trait ParserDispatcher<E: std::error::Error + std::marker::Send + std::marker::Sync + 'static>: clap::Parser {
    fn dispatch(&self) -> Result<(), E>;
    fn dispatch_cargo(&self) -> Result<(), E> {
        Ok(self.dispatch()?)
    }
    fn run() -> Result<(), E> {
        let (args, is_cargo) = Self::args();
        if is_cargo {
            Self::dispatch_cargo(&Self::parse_from(&args))?;
        } else {
            Self::dispatch(&Self::parse_from(&args))?;
        }
        Ok(())
    }
    fn main() -> Result<(), color_eyre::Report> {
        Self::run()?;
        Ok(())
    }
    fn args() -> (Vec<String>, bool) {
        let args = iocore::env::args();
        let execname = iocore::Path::new(&args[0]).name();
        let is_cargo = execname.ends_with("cargo") || execname.ends_with("cargo-upgrade");
        let args = if is_cargo { args[1..].to_vec() } else { args.to_vec() };
        (args, is_cargo)
    }
}
