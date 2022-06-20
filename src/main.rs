use fail_to_interface::PluginService;
mod test_helpers;

struct FailToPwnEngine {
    plugins: Vec<Box<dyn PluginService>>,
}

impl FailToPwnEngine {
    pub fn new() -> Self {
        FailToPwnEngine {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn PluginService>) {
        self.plugins.push(plugin);
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::create_service;
    use crate::FailToPwnEngine;

    #[test]
    fn engine_successful_init() {
        let plugin = create_service();

        let mut engine = FailToPwnEngine::new();
        engine.add_plugin(plugin);
    }
}
