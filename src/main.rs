use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

struct TemplateEngine {
    templates_dir: PathBuf,
    pages_dir: PathBuf,
    base_template: String,
}

impl TemplateEngine {
    fn new<P: AsRef<Path>>(
        templates_dir: P,
        pages_dir: P,
        base_template_path: P,
    ) -> Result<Self, Box<dyn Error>> {
        let base_template = fs::read_to_string(&base_template_path)?;
        Ok(TemplateEngine {
            templates_dir: templates_dir.as_ref().to_path_buf(),
            pages_dir: pages_dir.as_ref().to_path_buf(),
            base_template,
        })
    }

    fn render_page(&self, template_name: &str) -> Result<String, Box<dyn Error>> {
        let template_path = self.templates_dir.join(template_name);
        let template_content = fs::read_to_string(template_path)?;
        // Extract title from the template content
        let title = if let Some(title_end) = template_content.find('\n') {
            template_content[..title_end].trim().to_string()
        } else {
            "Untitled".to_string()
        };
        // Remove the title line from the content
        let content = template_content
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");
        let mut full_page = self.base_template.replace("{{title}}", &title);
        full_page = full_page.replace("{{body}}", &content);
        Ok(full_page)
    }

    fn generate_pages(&self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.pages_dir)?;
        for entry in fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
                let template_name = path.file_name().unwrap().to_str().unwrap();
                let rendered_page = self.render_page(template_name)?;
                let output_path = self.pages_dir.join(template_name);
                fs::write(output_path, rendered_page)?;
            }
        }
        println!("Pages generated successfully!");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args.len() > 5 {
        eprintln!(
            "Usage: {} <templates_dir> <pages_dir> <base_template.html> [-w]",
            args[0]
        );
        std::process::exit(1);
    }

    let templates_dir = &args[1];
    let pages_dir = &args[2];
    let base_template = &args[3];
    let watch_mode = args.get(4).map_or(false, |arg| arg == "-w");

    let engine = TemplateEngine::new(templates_dir, pages_dir, base_template)?;

    // Initial generation
    engine.generate_pages()?;

    if watch_mode {
        println!("Watch mode enabled. Watching for changes...");
        // Set up file watcher
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(2))?;
        watcher.watch(templates_dir, RecursiveMode::Recursive)?;
        println!(
            "Watching for changes in {}. Press Ctrl+C to stop.",
            templates_dir
        );
        loop {
            match rx.recv() {
                Ok(_) => {
                    println!("Change detected, regenerating pages...");
                    if let Err(e) = engine.generate_pages() {
                        eprintln!("Error regenerating pages: {}", e);
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
    } else {
        println!("Pages generated. Run with -w flag to enable watch mode.");
    }

    Ok(())
}
