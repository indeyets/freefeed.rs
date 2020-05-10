use freefeed::api::Post;

pub fn format_post(val: Post) {
    println!("From: {} ({}):\n", val.author.screen_name, val.author.username);
    println!("{}", val.body);

    if val.attachments.len() > 0 {
        println!("\nAttachments:");
        for attachment in val.attachments {
            println!("- {} ({} bytes)", attachment.file_name, attachment.file_size);
            println!("  {}", attachment.url);
        }
    }
}
