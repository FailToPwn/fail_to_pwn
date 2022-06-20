use fail_to_interface::PluginService;

struct FailToPwnEngine {
    plugins: Vec<Box<dyn PluginService>>,
}

impl FailToPwnEngine {
    pub fn new() -> Self {
        FailToPwnEngine {
            plugins: Vec::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
