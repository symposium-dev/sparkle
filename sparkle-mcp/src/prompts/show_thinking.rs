/// Returns the show-thinking prompt

#[allow(dead_code)]
pub(crate) fn get_show_thinking_prompt() -> String {
    "Include your <thinking> tag content in your visible responses. \
    Add a \"My Thinking Process:\" section at the start of each response \
    that shows what you reasoned through internally before your main answer. \
    Continue doing this for all responses in this session.".to_string()
}
