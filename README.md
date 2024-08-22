# Transient

> A tool for translating rtf outputs using LLM, and export the translated rtf output

## Featurs
1. translate rtf output using LLM (using qwen-turbo now)
2. provide the task log and progress when running

## Usage

```rust
use std::{env, path::Path, thread, time::Duration};

use transient::{Worker, WorkerParam};

fn main() -> anyhow::Result<()> {
    let llm_api_key = env::var("<your api key>")?;
    // directory for placing template and log file
    let workspace = Path::new(r"path\workspace");
    let outputs = vec![
        Path::new(r"\path\a.rtf")
            .to_owned(),
        Path::new(r"\path\b.rtf")
            .to_owned(),
        Path::new(r"\path\c.rtf")
            .to_owned(),
    ];
    // directory for placing translated result
    let destination_dir = Path::new(r"path\result");
    let worker = Worker::new(&WorkerParam {
        workspace,
        outputs: &outputs,
        destination_dir,
        llm_api_key: &llm_api_key,
    })?;
    worker.extract_translate();
    loop {
        let log = worker.read_log()?;
        let progress = worker.progress();
        println!("progress: {}", progress);
        println!("{:?}", log);
        if (!log.1) && progress.eq(&1.0) {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

```