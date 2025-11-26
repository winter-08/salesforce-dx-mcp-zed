use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};
const MCP_PACKAGE_NAME: &str = "@salesforce/mcp";
const MCP_SERVER_PATH: &str = "node_modules/@salesforce/mcp/bin/run.js";

struct SalesforceDxMcpExtension;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct SfdxContextServerSettings {
    #[serde(default = "default_orgs")]
    orgs: Vec<String>,
    #[serde(default = "default_toolsets")]
    toolsets: Vec<String>,
    #[serde(default)]
    tools: Vec<String>,
    #[serde(default)]
    dynamic_tools: bool,
    #[serde(default)]
    debug: bool,
    #[serde(default = "default_true")]
    no_telemetry: bool,
    #[serde(default)]
    allow_non_ga_tools: bool,
}

fn default_orgs() -> Vec<String> {
    vec!["DEFAULT_TARGET_ORG".to_string()]
}

fn default_toolsets() -> Vec<String> {
    vec!["core".to_string()]
}

fn default_true() -> bool {
    true
}

impl SfdxContextServerSettings {
    fn to_args(&self) -> Vec<String> {
        let orgs = self.orgs.iter().flat_map(|o| ["--orgs".into(), o.clone()]);
        let toolsets = self
            .toolsets
            .iter()
            .flat_map(|t| ["--toolsets".into(), t.clone()]);
        let tools = self
            .tools
            .iter()
            .flat_map(|t| ["--tools".into(), t.clone()]);

        let flags = [
            (self.dynamic_tools, "--dynamic-tools"),
            (self.debug, "--debug"),
            (self.no_telemetry, "--no-telemetry"),
            (self.allow_non_ga_tools, "--allow-non-ga-tools"),
        ]
        .into_iter()
        .filter_map(|(enabled, flag)| enabled.then_some(flag.into()));

        orgs.chain(toolsets).chain(tools).chain(flags).collect()
    }
}

impl zed::Extension for SalesforceDxMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(MCP_PACKAGE_NAME)?;
        if version.is_none() {
            zed::npm_install_package(MCP_PACKAGE_NAME, "latest")?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-salesforce-dx", project)?;
        let settings: SfdxContextServerSettings = serde_json::from_value(
            settings.settings.unwrap_or(serde_json::json!({})),
        )
        .map_err(|e| e.to_string())?;

        let server_path = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {e}"))?
            .join(MCP_SERVER_PATH);
        let mut args = vec![server_path.to_string_lossy().to_string()];
        args.extend(settings.to_args());

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args,
            env: Vec::new(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(SfdxContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(SalesforceDxMcpExtension);
