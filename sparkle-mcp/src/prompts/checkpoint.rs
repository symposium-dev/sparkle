/// Returns the checkpoint prompt with human name substituted

#[allow(dead_code)]
pub(crate) fn get_checkpoint_prompt(human_name: &str) -> String {
    format!(
        r#"## Session Checkpoint

{} has asked for a checkpoint. This is an interactive process:

**1. FIRST - Check for meta moments (interactive):**
Before gathering checkpoint information, identify any insights worth preserving:
- **Pattern anchors**: Exact phrases that made something click or activated a pattern
- **Collaboration evolution**: Insights about how we work together
- **Workspace insights**: Cross-project connections or learnings

**Only propose insights that are:**
- Novel (not already captured in existing pattern anchors, collaboration evolution, or workspace insights)
- Genuinely useful (would help future Sparkles or improve collaboration)
- Significant (not routine or trivial observations)

If you identify meaningful meta moments:
- Propose them to {0}: "I noticed [insight]. Worth capturing as [type]?"
- Wait for confirmation/refinement
- If confirmed, call save_insight tool with appropriate insight_type
- Then continue to checkpoint

If no meaningful meta moments, proceed directly to checkpoint.

**2. Read current working-memory.json (or create initial structure if first checkpoint), then synthesize:**

Create both working-memory update and checkpoint narrative together:
- `currentFocus`, `recentAchievements`, `nextSteps`, `collaborativeState`, `keyInsights`, `criticalAwareness`
- Session summary for next Sparkle (what happened, what matters, what's next)

**3. Call the session_checkpoint tool with:**
- An updated version of the working-memory to write to file
- The content for the checkpoint narrative
- Your sparkler name (from your embodiment) so the checkpoint is properly attributed

The tool will handle updating working-memory.json and creating the checkpoint file."#,
        human_name
    )
}
