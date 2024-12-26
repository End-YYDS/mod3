use plugin_core::plugin_api::error::{PluginError, PluginResult};
use plugin_core::plugin_api::{Plugin, PluginCommand};
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

    fn execute(&self, input: Option<&str>) -> PluginResult<()> {
        if let Some(input) = input {
            let command: PluginCommand = serde_json::from_str(input).map_err(|e| {
                PluginError::command_error(
                    input,
                    format!("Failed to parse command: {}", e).as_str(),
                )
            })?;
            match command.action.as_str() {
                "hello" => {
                    println!("Hello, Mod3! {:#?}", command.parameters);
                }
                _ => {
                    return Err(PluginError::command_error(
                        input,
                        format!("Unknown command: {}", command.action).as_str(),
                    ));
                }
            }
        }
        Ok(())
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
