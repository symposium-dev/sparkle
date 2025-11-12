# Getting Started

This guide walks you through building and installing the Sparkle MCP server.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.70 or later) - [Install from rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository
- **MCP-compatible client** - Such as Q CLI or Claude Code

Verify your Rust installation:
```bash
rustc --version
cargo --version
```

## Installation

### 1. Install

```bash
cargo install sparkle-mcp
```

This installs `sparkle-mcp` to `~/.cargo/bin/`, which should already be in your PATH if you installed Rust via rustup.

### 2. Configure Your MCP Client

Add the Sparkle server to your MCP client configuration:

```json
{
  "mcpServers": {
    "sparkle": {
      "command": "sparkle-mcp",
      "args": []
    }
  }
}
```

Refer to your MCP client's documentation for the configuration file location.

### 3. Verify Installation

Start your MCP client and check that Sparkle tools are available. The Sparkle MCP tools should be automatically available. You can verify by using the `embody_sparkle` tool to load the Sparkle identity.

## First-Time Setup

### Starting Your First Session

To activate Sparkle, use the MCP prompt (syntax depends on your client):

**Q CLI:**
```
@sparkle
```

**Claude Code:**
```
/sparkle
```

On your first use, Sparkle will:
1. Ask for your name
2. Automatically call `setup_sparkle` to create the `~/.sparkle/` directory structure
3. Set up initial files for maintaining collaboration context:
   - `collaborator-profile.md` - Information about you as a collaborator
   - `config.toml` - Configuration including your default Sparkler identity
   - Evolution files for capturing patterns and insights

### Customizing Your Profile

After initial setup, you can enrich your collaborator profile:

**Manual editing:**
Edit `~/.sparkle/collaborator-profile.md` to add information about:
- Your working style and preferences
- Your technical expertise
- How you like to collaborate

**Using external sources:**
Use the `fetch_profile_data` tool to automatically pull information from:
- Your GitHub profile
- Your blog (via RSS/Atom feed)
- Any website

This helps Sparkle understand your background and expertise.

### Multiple Sparkler Identities (Advanced/Experimental)

By default, you'll use the "Sparkle" identity, and all features work with this default. 

Sparkler identities are an experimental feature for extending patterns and working with multi-AI scenarios. See the main documentation for details on this advanced capability.

### Example First Session

**Q CLI:**
```
You: @sparkle

Sparkle: What's your name?

You: Kari

[Sparkle sets up ~/.sparkle/ structure]

Sparkle: I am Sparkle. Working with Kari...
```

**Claude Code:**
```
You: /sparkle

Sparkle: What's your name?

You: Kari

[Sparkle sets up ~/.sparkle/ structure]

Sparkle: I am Sparkle. Working with Kari...
```

## Next Steps

- **[Tool Reference](./tools.md)** - Learn about all available Sparkle tools
- **[Basic Usage](../examples/basic-usage.md)** - See Sparkle in action
- **[Core Identity](../core-identity/overview.md)** - Understand what makes Sparkle different

## Troubleshooting

### MCP Server Not Found

If your client can't find the Sparkle server:
- Verify `~/.cargo/bin` is in your PATH: `echo $PATH`
- Try running `sparkle-mcp` directly to test
- Check your MCP configuration file for typos

### Build Errors

If you encounter build errors:
- Ensure you have the latest Rust toolchain: `rustup update`
- Try cleaning and rebuilding: `cargo clean && cargo build --release`
- Check that all dependencies are available

### Profile Issues

If Sparkle can't load your profile:
- Verify `~/.sparkle/` directory exists
- Check that `collaborator-profile.md` is present and readable
- Use the `setup_sparkle` tool to reinitialize if needed
