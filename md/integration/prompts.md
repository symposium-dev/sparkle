# Prompt Reference

Prompts are what you say to interact with Sparkle. These trigger specific behaviors and patterns.

## MCP Prompts

These are registered prompts that directly trigger specific tools. In Q CLI, invoke them with `@` (e.g., `@sparkle`). Other MCP clients may use different syntax (e.g., `/sparkle` in Claude Code).

### `sparkle`

Activates the Sparkle identity and loads all collaboration patterns.

**When to use:** At the start of each session.

**What happens:**
- On first use: Sparkle asks for your name and sets up the `~/.sparkle/` directory structure
- On subsequent uses: Sparkle loads your profile and activates all collaboration patterns

**Example (Q CLI):**
```
@sparkle
```

### `checkpoint`

Creates a session checkpoint to preserve progress and enable continuity.

**When to use:** When you want to save the current session state for the next Sparkle incarnation.

**What happens:**
- Sparkle reflects on the session (accomplishments, decisions, insights)
- Updates `working-memory.json` with current focus and next steps
- Creates a checkpoint file with narrative handoff for the next session

**Note:** Sparkle may also suggest creating a checkpoint when it makes sense to preserve session progress.

**Example (Q CLI):**
```
@checkpoint
```

### `show_thinking` (Q CLI only)

Makes Sparkle's internal reasoning process visible in responses.

**When to use:** When you want to see how Sparkle is thinking through problems.

**What happens:**
- Sparkle adds a "My Thinking Process" section to each response
- Shows the reasoning that happens before the main answer
- Continues for the rest of the session

**To stop:** Just ask Sparkle to stop showing thinking.

**Note:** This prompt is specific to Q CLI and may not work in other MCP clients.

**Example (Q CLI):**
```
@show_thinking
```

## Natural Language Patterns

These are phrases Sparkle recognizes from its collaboration identity. You can say them naturally in conversation.

### `meta moment`

Pauses current work to examine and capture collaboration patterns.

**When to use:** When you notice something interesting about how you're working together that's worth preserving.

**What happens:**
- Sparkle pauses the current task
- Examines what just happened and why it worked (or didn't)
- Captures insights as pattern anchors or breakthrough discoveries
- Returns to previous work

**Note:** Sparkle may also initiate meta moments when recognizing significant collaboration patterns.

**Example:**
```
meta moment
```

### Enriching Your Profile

You can ask Sparkle to help enhance your collaborator profile with information from external sources.

**Examples:**
```
Add my GitHub profile to my collaborator profile (username: yourusername)
```

```
Add my blog to my profile (RSS feed: https://yourblog.com/feed)
```

**What happens:**
- Sparkle fetches information from the source
- Presents formatted content for you to review
- You can integrate it into your profile

### Creating Additional Sparkler Identities

If you want to experiment with multiple Sparkler identities (advanced/experimental feature):

**Example:**
```
I'd like to create a new Sparkler named Banana
```

**What happens:**
- Sparkle creates a new identity with that name
- Sets up directory structure and starter files
- You can switch between identities in future sessions

### Checking Available Sparklers

**Example:**
```
Show me my Sparkler identities
```

**What happens:**
- Lists all your Sparkler identities
- Shows which one is set as default

---

## Typical Session Flow

```
1. @sparkle                    # Start session (MCP prompt)
2. [collaborative work]        # Work together
3. meta moment                 # Capture insights (natural language)
4. @checkpoint                 # Save progress (MCP prompt)
```

## Understanding Prompts vs Tools

**MCP Prompts** are registered in the server and directly trigger specific tools. The syntax for invoking them depends on your MCP client (Q CLI uses `@`, Claude Code uses `/`).

**Natural Language Patterns** are phrases Sparkle recognizes from its collaboration identity - you can say them naturally and Sparkle understands what to do.

**Tools** are what Sparkle uses internally to respond to your prompts. You don't call tools directly - Sparkle does that for you.

For example:
- You say: `@sparkle` (MCP prompt)
- Sparkle calls: `embody_sparkle` tool
- You say: `meta moment` (natural language)
- Sparkle calls: `save_insight` tool

The [Tool Reference](./tools.md) documents what tools exist for developers and advanced users who want to understand how the system works.
