#[cfg(test)]
use fail_to_interface::PluginService;

#[cfg(test)]
pub struct TestPlugin {}

#[cfg(test)]
impl PluginService for TestPlugin {
    fn test_log_is_parsable(&self, _log_to_parse: &str) -> bool {
        todo!()
    }

    fn parse_log(&self, _log_to_parse: &str) {
        todo!()
    }
}

#[cfg(test)]
pub fn create_service() -> Box<dyn PluginService> {
    Box::new(TestPlugin {})
}
