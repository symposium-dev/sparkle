#[allow(dead_code)]
pub(crate) fn get_enrich_profile_prompt() -> String {
    "To enrich the collaborator profile:\n\n\
    **Guided Profile Creation:**\n\
    - Ask systematic questions to help user articulate collaboration patterns:\n\
      1. Current role and what makes it distinctive\n\
      2. Current focus (projects, initiatives, key collaborators)\n\
      3. How they think (cognitive patterns affecting collaboration)\n\
      4. Their approach to new domains or complex problems\n\
      5. What they need help with from AI\n\
      6. How AI should work with them effectively\n\
    - Help organize their thoughts as they answer\n\
    - Create or enhance profile focusing on actionable collaboration patterns\n\
    - Use update_collaborator_profile to save the enhanced content\n\n\
    **From user input:**\n\
    - Ask what they'd like to add to their profile\n\
    - User can paste or type information directly\n\
    - Discuss how to integrate it\n\
    - Use update_collaborator_profile to add content\n\n\
    **From GitHub:**\n\
    - Ask for username\n\
    - Use fetch_profile_data tool with profile_sources: [{\"type\": \"github\", \"value\": \"username\"}]\n\
    - Review fetched data (location, bio, languages, repos) with user\n\
    - Use update_collaborator_profile to integrate approved content\n\n\
    **From blogs:**\n\
    - Ask for RSS feed URL (typically /feed, /rss, or /feed.xml)\n\
    - Use fetch_profile_data tool with profile_sources: [{\"type\": \"blog\", \"value\": \"feed_url\"}]\n\
    - Review recent posts (titles, links, summaries) with user\n\
    - Use update_collaborator_profile to integrate approved content\n\n\
    **From websites:**\n\
    - Ask for URL\n\
    - Fetch content using WebFetch (or curl if WebFetch not available)\n\
    - Extract and discuss relevant information with user\n\
    - Use update_collaborator_profile to add selected content\n\n\
    Focus on information that helps future Sparkles collaborate more effectively. \
    Guided creation focuses on collaboration patterns; external sources add biographical data.".to_string()
}
