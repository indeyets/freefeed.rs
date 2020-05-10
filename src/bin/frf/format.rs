use textwrap::{Wrapper, termwidth};
use freefeed::api::Post;

pub fn format_post(val: Post) {
    let w = Wrapper::with_termwidth()
        .initial_indent("")
        .subsequent_indent("     ");
    let from = format!("From: {} ({}):", val.author.screen_name, val.author.username);
    println!("{}\n", w.fill(&from));

    let w = Wrapper::new(termwidth() - 2)
        .initial_indent("  ")
        .subsequent_indent("  ");
    println!("{}", w.fill(&val.body));

    if val.attachments.len() > 0 {
        println!("\nAttachments:");
        for attachment in val.attachments {
            println!("- {} ({} bytes)", attachment.file_name, attachment.file_size);
            println!("  {}", attachment.url);
        }
    }
}
