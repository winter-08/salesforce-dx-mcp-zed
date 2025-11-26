# Salesforce DX MCP Server for Zed

A [Zed](https://zed.dev) extension that integrates the [Salesforce DX Model Context Protocol (MCP) server](https://developer.salesforce.com/docs/atlas.en-us.sfdx_dev.meta/sfdx_dev/sfdx_dev_mcp_server.htm) for interacting with Salesforce orgs.

## Prerequisites

- [Salesforce CLI](https://developer.salesforce.com/tools/salesforcecli) installed and configured
- At least one authenticated Salesforce org

## Installation

Install from the Zed extension marketplace by searching for "Salesforce DX MCP Server".

## Configuration

Configure the extension in your Zed settings under `context_servers`:

```json
{
  "context_servers": {
    "mcp-server-salesforce-dx": {
      "settings": {
        "orgs": ["DEFAULT_TARGET_ORG"],
        "toolsets": ["core"],
        "tools": [],
        "dynamic_tools": false,
        "debug": false,
        "no_telemetry": true,
        "allow_non_ga_tools": false
      }
    }
  }
}
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `orgs` | `string[]` | `["DEFAULT_TARGET_ORG"]` | Org aliases to connect to |
| `toolsets` | `string[]` | `["core"]` | Tool categories to enable (e.g., `metadata`, `data`, `orgs`, `users`) |
| `tools` | `string[]` | `[]` | Specific tools to enable in addition to toolsets |
| `dynamic_tools` | `bool` | `false` | Enable dynamic tool discovery |
| `debug` | `bool` | `false` | Enable debug logging |
| `no_telemetry` | `bool` | `true` | Disable telemetry |
| `allow_non_ga_tools` | `bool` | `false` | Enable non-GA (beta) tools |

For more details on configuration options, see the [Salesforce documentation](https://developer.salesforce.com/docs/atlas.en-us.sfdx_dev.meta/sfdx_dev/sfdx_dev_mcp_server.htm#sfdx_dev_mcp_configure_orgs_toolsets).
