use color_print::cprintln;

pub fn info(s: String) {
    cprintln!("<green>INFO</green> - {}", s)
}
pub fn err(s: String) {
    cprintln!("<red>ERR</red> - {}", s)
}
// pub fn info(s: String) {
//     cprintln!("<green>INFO</green> - {}", s)
// }
