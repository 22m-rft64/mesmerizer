pub fn render(template: &str, context: &str) -> String {
    template.replace("{context}", context)
}
