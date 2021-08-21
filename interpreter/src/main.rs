use codespan_reporting as cr;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Only check for parse errors
    #[structopt(short, long)]
    check: bool,
}

fn main() {
    let opt = Opt::from_args();

    let path = opt.input;
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("error: cannot read {}", path.display());
        eprintln!("    {}", e);
        std::process::exit(1);
    });

    match interpreter::parse(&source) {
        Ok(_ast) => {
            // eprintln!("ast: {:#?}", ast);
        }
        Err(e) => {
            if print_diagnostics(
                &path.to_string_lossy(),
                &source,
                std::iter::once(e),
            )
                .is_err()
            {
                std::process::exit(1);
            }
        }
    }

    if opt.check {
        return;
    }
}

fn print_diagnostics(
    file: &str,
    source: &str,
    diagnostics: impl Iterator<Item = interpreter::Error>,
) -> anyhow::Result<()> {
    let stream = cr::term::termcolor::StandardStream::stderr(cr::term::termcolor::ColorChoice::Auto);
    let mut stream = stream.lock();

    let chars = codespan_reporting::term::Chars::ascii();

    let config = codespan_reporting::term::Config {
        chars,
        .. codespan_reporting::term::Config::default()
    };
    let mut files = cr::files::SimpleFiles::new();
    let file = files.add(file, source);
    for diagnostic in diagnostics {
        let severity = cr::diagnostic::Severity::Error;
        let diagnostic = cr::diagnostic::Diagnostic::new(severity)
            .with_message(diagnostic.message)
            .with_labels(vec![
                cr::diagnostic::Label::primary(file, diagnostic.span.source_range()),
            ]);

        cr::term::emit(&mut stream, &config, &files, &diagnostic)?;
    }
    Ok(())
}
