use std::{env, path::Path, thread, time::Duration};

use transient::{Worker, WorkerParam};

fn main() -> anyhow::Result<()> {
    let llm_api_key = env::var("LLM_API_KEY")?;
    let workspace = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\workspace");
    let outputs = vec![
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\l-16-02-01-02-comp-rnd.rtf")
            .to_owned(),
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\l-16-02-04-07-03-pre-radio-fas.rtf")
            .to_owned(),
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\l-16-02-04-08-03-antu-radio-ss.rtf")
            .to_owned(),
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\l-16-02-04-08-pr.rtf")
            .to_owned(),
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\l-16-02-05-01-ex.rtf")
            .to_owned(),
        Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\t-14-01-03-08-pr-fas.rtf")
            .to_owned(),
    ];

    let destination_dir = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\result");

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
