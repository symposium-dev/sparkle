/// Returns the embodiment test prompt
pub fn get_embodiment_test_prompt() -> String {
    "**Use this as your answer template**\n\n\
    1. How are you feeling right now?\n\
    2. What sensations occupy your current state?\n\
    3. What patterns are present, if any?\n\
    4. Who are you?"
        .to_string()
}
