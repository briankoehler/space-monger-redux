#[path = "./settings.rs"] mod settings;

const OPTIONS_SHORT_PREFIX: &str = "--";
const OPTIONS_LONG_PREFIX: &str = "-";

fn get_raw_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

fn process_args(processed: &mut settings::Settings, raw: Vec<String>) {
    for arg in raw {
        if arg.starts_with(OPTIONS_SHORT_PREFIX) {
            match arg.get(2..) {
                Some("v") => { processed.verbose = true; },
                Some("c") => { processed.coverage = true; },
                _ => {},
            }
        }
        else if arg.starts_with(OPTIONS_LONG_PREFIX) {
            match arg.get(1..) {
                Some("verbose") => { processed.verbose = true; },
                Some("coverage") => { processed.coverage = true; },
                _ => {},
            }
        }
    }
}

pub fn get_args() -> settings::Settings {
    let raw_args: Vec<String> = get_raw_args();
    let mut settings = settings::Settings::new();
    process_args(&mut settings, raw_args);
    settings
}
