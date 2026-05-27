/// 24x11 ANSI truecolor half-block sprite. Embedded at compile time so the binary is
/// self-contained. Emitted at the top of every `mes teto *` text output (skipped in --json).
const SPRITE: &str = include_str!("../../../../assets/teto-ascii/teto.txt");

/// Write the sprite to stdout. Caller decides when to call.
pub fn print_sprite() {
    print!("{}", SPRITE);
}

/// Same as print_sprite but to any writer (for testing).
pub fn write_sprite<W: std::io::Write>(w: &mut W) -> std::io::Result<()> {
    write!(w, "{}", SPRITE)
}

pub fn sprite() -> &'static str {
    SPRITE
}

#[cfg(test)]
mod tests {
    #[test]
    fn sprite_loads_and_has_ansi() {
        let s = super::sprite();
        assert!(s.contains("\x1b["), "sprite should contain ANSI escape codes");
        assert!(s.lines().count() >= 10, "sprite should be multi-line");
    }
}
