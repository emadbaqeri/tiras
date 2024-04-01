use crate::terminal::Terminal;

pub fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{:?}", e);
}
