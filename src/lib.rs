use plugin_core::plugin_api::Plugin;
use plugin_core::plugin_macro::*;
#[plugin_entry]
#[plugin_exit]
#[derive(Debug)]
struct Mod3;
const PLUGIN_NAME: &str = "Mod3";
const PLUGIN_VERSION: &str = "0.1.0";
const PLUGIN_DESCRIPTION: &str = "This is Mod3";
impl Plugin for Mod3 {
    fn name(&self) -> &str {
        PLUGIN_NAME
    }

    fn version(&self) -> &str {
        PLUGIN_VERSION
    }

    fn description(&self) -> &str {
        PLUGIN_DESCRIPTION
    }

    fn execute(&self, input: &str) -> String {
        format!("{}: {}", PLUGIN_NAME, input)
    }

    fn unload()
    where
        Self: Sized,
    {
        println!("[{}]: Unloading...", PLUGIN_NAME);
    }

    fn load() -> Box<dyn Plugin>
    where
        Self: Sized,
    {
        Box::new(Mod3)
    }
}
