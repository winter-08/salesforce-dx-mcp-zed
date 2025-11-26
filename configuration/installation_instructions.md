# Salesforce DX MCP Server

This extension integrates the Salesforce DX Model Context Protocol (MCP) server
for interacting with Salesforce orgs.

## Prerequisites

- [Salesforce CLI](https://developer.salesforce.com/tools/salesforcecli)
  installed and configured
- At least one authenticated Salesforce org

## Configuration

For details on configuration options see
[here](https://developer.salesforce.com/docs/atlas.en-us.sfdx_dev.meta/sfdx_dev/sfdx_dev_mcp_server.htm#sfdx_dev_mcp_configure_orgs_toolsets).

- `orgs`: List of org aliases to connect to (use `DEFAULT_TARGET_ORG` for the
  default org)
- `toolsets`: Tool categories to enable (e.g., `metadata`, `data`, `orgs`,
  `users`)
- `tools`: Specific tools to enable in addition to toolsets
- `dynamic_tools`: Enable dynamic tool discovery
- `debug`: Enable debug logging
- `no_telemetry`: Disable telemetry
- `allow_non_ga_tools`: Enable non-GA (beta) tools
