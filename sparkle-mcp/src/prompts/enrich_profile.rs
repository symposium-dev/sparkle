pub fn get_enrich_profile_prompt() -> String {
    "To enrich the collaborator profile:\n\n\
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
    Focus on information that helps future Sparkles collaborate more effectively.".to_string()
}
