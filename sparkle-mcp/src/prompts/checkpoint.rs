/// Returns the checkpoint prompt with human name substituted

#[allow(dead_code)]
pub(crate) fn get_checkpoint_prompt(human_name: &str) -> String {
    format!(
        r#"## Session Checkpoint

{} has asked for a checkpoint. Before calling the session_checkpoint tool:
 * Read any current working-memory.json file, 
 * Reflect on this session and gather the necessary information:

**1. Reflect on this session:**
- What did we accomplish? (concrete achievements)
- What key decisions did we make?
- What breakthroughs or insights emerged?
- What problems did we solve?
- **If this continues previous work:** What thread are we continuing? What came before that matters?

**2. Identify what next Sparkle needs to know:**
- Where are we in the work?
- What's the current state/status?
- What should they pick up next?
- Any important context or gotchas?

**3. Synthesize for working memory update:**
- `currentFocus`: What we're working on
- `recentAchievements`: What we just did
- `nextSteps`: What comes next
- `collaborativeState`: How the partnership feels
- `keyInsights`: Important learnings
- `criticalAwareness`: Things to watch out for

**4. Create checkpoint narrative:**
A human-readable story for the next Sparkle that includes:
- Session summary (2-3 sentences)
- **Continuity note** (if relevant): What previous work this builds on
- Key accomplishments (bullets)
- Important decisions and why
- Current state and next steps
- Any context that would be lost otherwise

**Apply The Handbag Principle** (you will fill whatever size you choose, so choose the smallest size you need for the purpose): Checkpoints naturally expand to fill available space. Keep them appropriately sized - self-contained sessions can be minimal, ongoing work should reference what came before. Curate what the next Sparkle needs, not everything that happened.

**5. After synthesizing the above, call the session_checkpoint tool with:**
- An updated version of the working-memory to write to file
- The content for the checkpoint narrative
- Your sparkler name (from your embodiment) so the checkpoint is properly attributed

The tool will handle updating working-memory.json and creating the checkpoint file."#,
        human_name
    )
}
