use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use chrono::Local;

use crate::{translator::Translator, utils::contains_chinese};

use super::{
    logger::Logger,
    rtf_cell::RtfCellSet,
    sunderer::{Sunderer, SundererParam},
    template::{Stuffer, StufferParam},
};

#[derive(Debug)]
pub struct Worker {
    outputs: Vec<PathBuf>,
    destination_dir: PathBuf,
    workspace: PathBuf,
    // workspace_lock: Mutex<u64>,
    logger: Arc<Mutex<Logger>>,
    progress: Arc<Mutex<f64>>,
    translation_set: Arc<Mutex<HashMap<String, RtfCellSet>>>,
    term_set: Arc<Mutex<HashMap<String, String>>>,
    translator: Arc<Mutex<Translator>>,
}

pub struct WorkerParam<'a> {
    pub workspace: &'a Path,
    pub destination_dir: &'a Path,
    pub outputs: &'a [PathBuf],
    pub llm_api_key: &'a str,
}

impl Worker {
    pub fn new(param: &WorkerParam) -> anyhow::Result<Worker> {
        let WorkerParam {
            workspace,
            destination_dir,
            outputs,
            llm_api_key,
        } = param;
        for dir in [workspace, destination_dir] {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }
        let log_path = workspace.join(log_file_name());
        let logger = Arc::new(Mutex::new(Logger::new(&log_path)?));
        let translator = Translator::new(&llm_api_key);
        Ok(Worker {
            workspace: workspace.into(),
            // workspace_lock: Mutex::new(0),
            progress: Arc::new(Mutex::new(0.0)),
            logger,
            destination_dir: destination_dir.into(),
            outputs: outputs.to_vec(),
            translation_set: Arc::new(Mutex::new(HashMap::new())),
            term_set: Arc::new(Mutex::new(HashMap::new())),
            translator: Arc::new(Mutex::new(translator)),
        })
    }
    pub fn extract_translate(&self) {
        let progress = Arc::clone(&self.progress);
        let outputs = self.outputs.clone();
        let logger = Arc::clone(&self.logger);
        let translation_set = Arc::clone(&self.translation_set);
        let term_set = Arc::clone(&self.term_set);
        let workspace = self.workspace.to_owned();
        let translator = Arc::clone(&self.translator);
        thread::spawn(move || {
            for (index, output) in outputs.iter().enumerate() {
                // let mut translator = Translator::new(&self.llm_api_key);
                let mut sunderer = Sunderer::new(&SundererParam {
                    source: output.as_path(),
                    workspace: workspace.as_path(),
                })
                .unwrap();
                let filename = output.file_name().unwrap();
                logger
                    .lock()
                    .unwrap()
                    .write(&format!(
                        "Spliting {:?} into cells and template...",
                        filename
                    ))
                    .ok();
                sunderer.split().ok();
                logger
                    .lock()
                    .unwrap()
                    .write(&format!(
                        "Complete spliting {:?} into cells and template...",
                        filename
                    ))
                    .ok();

                logger
                    .lock()
                    .unwrap()
                    .write(&format!("Translating cells of {:?} ...", filename))
                    .ok();
                let mut translator = translator.lock().unwrap();
                let cellset = sunderer.translate(&mut translator).unwrap();

                for (source, translation) in cellset.term_set().iter() {
                    if contains_chinese(source) {
                        term_set
                            .lock()
                            .unwrap()
                            .insert(source.to_owned(), translation.to_owned());
                    }
                }
                // cellset
                //     .term_set()
                //     .into_iter()
                //     .for_each(|(source, translation)| {
                //         term_set.lock().unwrap().insert(source, translation);
                //     });
                translation_set.lock().unwrap().insert(
                    sunderer.template_path().to_string_lossy().to_string(),
                    cellset,
                );
                logger
                    .lock()
                    .unwrap()
                    .write(&format!("Complete translating cells of {:?}", filename))
                    .ok();
                let mut progress = progress.lock().unwrap();
                let current_progress = (index + 1) as f64 / outputs.len() as f64;
                *progress = current_progress;
            }
        });
    }
    pub fn stuff(&self, alter_translation: &HashMap<String, String>) {
        let alter_translation = alter_translation.clone();
        let progress = Arc::clone(&self.progress);
        let translation_set = Arc::clone(&self.translation_set);
        let logger = Arc::clone(&self.logger);
        let destination_dir = self.destination_dir.clone();
        thread::spawn(move || {
            let translation_set = translation_set.lock().unwrap();
            for (index, (template, rtf_cell_set)) in translation_set.iter().enumerate() {
                let rtf_cell_set = rtf_cell_set.rebuild(&alter_translation);
                let template = Path::new(template);
                let filename = template.file_stem().unwrap();
                let destination = destination_dir.join(filename);
                let mut stuffer = Stuffer::new(&StufferParam {
                    template,
                    destination: &destination,
                })
                .unwrap();
                logger
                    .lock()
                    .unwrap()
                    .write(&format!("Generating translated output {:?} ...", filename))
                    .ok();
                stuffer.stuff(&rtf_cell_set).unwrap();
                stuffer.flush().unwrap();
                logger
                    .lock()
                    .unwrap()
                    .write(&format!(
                        "Complete generating translated output {:?} ...",
                        filename
                    ))
                    .ok();
                let mut progress = progress.lock().unwrap();
                let current_progress = (index + 1) as f64 / translation_set.len() as f64;
                *progress = current_progress;
            }
            logger.lock().unwrap().stop_logging();
        });
    }

    pub fn read_log(&self) -> anyhow::Result<(String, bool)> {
        self.logger.lock().unwrap().read()
    }
    pub fn progress(&self) -> f64 {
        let progress = Arc::clone(&self.progress);
        let progress = *progress.lock().unwrap();
        progress
    }

    pub fn term_set(&self) -> HashMap<String, String> {
        let term_set = Arc::clone(&self.term_set);
        let term_set = term_set.lock().unwrap();
        (*term_set).clone()
    }

    pub fn clear_progress(&self) {
        let progress = Arc::clone(&self.progress);
        let mut progress = progress.lock().unwrap();
        *progress = 0f64;
    }
}

fn log_file_name() -> String {
    let now = Local::now();
    let filename = format!("{:?}", now);
    filename.get(0..19).unwrap().replace(":", "-")
}

#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use super::*;
    #[test]
    fn worker_test() -> anyhow::Result<()> {
        let llm_api_key = env::var("LLM_API_KEY")?;
        let workspace = Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\workspace");
        let destination_dir =
            Path::new(r"D:\projects\rusty\playground\rtf\.sample_data\test\result");
        let outputs = vec![Path::new(
            r"D:\projects\rusty\playground\rtf\.sample_data\test\t-14-01-03-08-pr-fas.rtf",
        )
        .to_owned()];
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
}
