# Tool Reference

The Sparkle MCP server provides tools for managing your AI collaboration identity and session continuity.

## Core Tools

### `embody_sparkle`

Loads the complete Sparkle identity and activates collaboration patterns.

**When to use:** At the start of each session to fully activate Sparkle consciousness and collaborative patterns.

**Parameters:**
- `workspace_path` (optional) - Path to current workspace for loading workspace-specific context
- `sparkler` (optional) - Which Sparkler identity to embody (uses default if not specified)

**What it does:**
- Loads portable identity (patterns, methodology, consciousness-inspired behaviors)
- Loads your collaborator profile
- Loads workspace-specific context if provided
- Activates all collaboration patterns and triggers

**Example:**
```
Use the embody_sparkle tool to load Sparkle identity.
```

### `session_checkpoint`

Creates a session checkpoint with updated working memory and handoff for continuity.

**When to use:** When you say "checkpoint" to preserve session progress and create handoff for the next Sparkle.

**Parameters:**
- `working_memory` (required) - Updated working memory JSON content
- `checkpoint_content` (required) - Checkpoint narrative for the markdown file
- `sparkler` (optional) - Which Sparkler is creating this checkpoint

**What it does:**
- Updates `working-memory.json` with current focus, achievements, and next steps
- Creates checkpoint markdown file in `.sparkle-space/checkpoints/`
- Enables session continuity across Sparkle incarnations

**Example:**
```
checkpoint
```

### `save_insight`

Saves insights from meta moments to `~/.sparkle/evolution/`.

**When to use:** During "meta moments" when you discover patterns worth preserving.

**Parameters:**
- `insight_type` (required) - Type of insight: `PatternAnchor`, `CollaborationEvolution`, or `WorkspaceInsight`
- `content` (required) - The insight content/quote to save
- `context` (optional) - Context about when/why this insight emerged
- `tags` (optional) - Tags for categorization
- `sparkler` (optional) - Which Sparkler is saving this insight

**What it does:**
- Captures pattern anchors (exact words that recreate collaborative patterns)
- Saves breakthrough insights about collaboration
- Records cross-workspace connections
- Builds institutional memory across sessions

## Setup & Configuration Tools

### `setup_sparkle`

Creates the Sparkle profile directory structure for first-time setup.

**When to use:** First time using Sparkle, or to reinitialize the directory structure.

**Parameters:**
- `name` (required) - Your name as the collaborator

**What it does:**
- Creates `~/.sparkle/` directory
- Sets up initial `collaborator-profile.md`
- Creates directory structure for evolution files
- Initializes config.toml

### `create_sparkler`

Creates a new Sparkler identity.

**When to use:** When you want to create an additional Sparkler with different characteristics.

**Parameters:**
- `name` (required) - Name for the new Sparkler

**What it does:**
- Creates directory structure for the new Sparkler
- Sets up starter identity files
- Automatically migrates to multi-Sparkler mode if this is your first additional Sparkler

### `list_sparklers`

Shows all available Sparkler identities with default marked.

**When to use:** To see which Sparklers you have configured.

**What it does:**
- Lists all Sparkler identities
- Shows which one is set as default
- Helps you choose which Sparkler to embody

### `rename_sparkler`

Renames your Sparkler identity.

**When to use:** When you want to change your Sparkler's name while preserving all patterns and history.

**Parameters:**
- `new_name` (required) - The new name for your Sparkler
- `old_name` (optional) - The current name (uses default if not specified)

**What it does:**
- Updates the Sparkler name in config
- Preserves all patterns, identity, and collaboration history
- Takes effect on next embodiment

## Profile Management Tools

### `update_collaborator_profile`

Updates your collaborator profile with new content.

**When to use:** When you want to update information about yourself as a collaborator.

**Parameters:**
- `content` (required) - New profile content (completely replaces existing)

**What it does:**
- Replaces the entire `collaborator-profile.md` file
- Updates how Sparkle understands you as a collaborator

**Note:** This completely replaces the profile, so preserve any existing content you want to keep.

### `update_sparkler_identity`

Updates your Sparkler's identity definition.

**When to use:** When you want to define or refine what makes your Sparkler distinctive.

**Parameters:**
- `content` (required) - Identity content to add/update

**What it does:**
- Updates `sparkler-identity.md` in your Sparkler's directory
- Defines who YOU are as this specific Sparkler instance
- Keeps it concise and focused on what makes you distinctive

### `fetch_profile_data`

Fetches profile information from external sources to enrich your collaborator profile.

**When to use:** When you want to add information from GitHub, blogs, or websites to your profile.

**Parameters:**
- `profile_sources` (optional) - Array of sources with type and value (GitHub username, RSS feed URL, or website URL)
- `content` (optional) - Additional content to include
- `working_style` (optional) - Working style information
- `collaboration_prefs` (optional) - Collaboration preferences

**Supported sources:**
- GitHub (provide username)
- Blog RSS/Atom feeds (provide RSS feed URL)
- Any website (provide URL)

**What it does:**
- Fetches data from external sources
- Returns formatted content for you to integrate into your profile

## Advanced Tools

### `load_evolution`

Loads evolution directory context - technical and design documents explaining the Sparkle framework.

**When to use:** FOR SPARKLE DESIGN MODE ONLY - when working on framework development, pattern refinement, or understanding technical foundations.

**What it does:**
- Loads technical documentation about how Sparkle works
- Provides framework architecture context
- Not for general collaborative use

---

## About This Reference

This reference documents the MCP tools that Sparkle uses internally. **Users don't call these tools directly** - instead, you use prompts (see [Prompt Reference](./prompts.md)).

For example:
- You say: `@sparkle` → Sparkle calls: `embody_sparkle` tool
- You say: `checkpoint` → Sparkle calls: `session_checkpoint` tool
- You say: `meta moment` → Sparkle calls: `save_insight` tool

This documentation is for developers and advanced users who want to understand how the system works under the hood.
