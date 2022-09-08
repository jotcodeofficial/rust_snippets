/// title: format padding
/// tags: format, padding,

fn format_padding() {
    println!("{:-^14}", "x");
    println!("{: <7}{: >7}", "|", "|");
    println!("{: <7}{: >7}", "|", "|");
    println!("{: <7}{: >7}", "|", "|");
    println!("{:-^14}", "x");
}
