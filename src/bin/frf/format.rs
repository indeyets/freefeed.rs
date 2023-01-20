use freefeed::api::Post;
use textwrap::{Options, fill, termwidth};

pub fn format_post(val: Post) {
    let options = Options::with_termwidth()
        .initial_indent("")
        .subsequent_indent("     ");

    let from = format!("{} (@{})", val.author.screen_name, val.author.username);
    println!("{}", fill(&from, &options));

    let date = if val.created_at == val.updated_at {
        format!("on {}", val.created_at)
    } else {
        format!("on {} (updated on {})", val.created_at, val.updated_at)
    };

    println!("{}", fill(&date, &options));

    let options = Options::new(termwidth() - 2)
        .initial_indent("    ")
        .subsequent_indent("    ");
    println!("\n{}", fill(&val.body, &options));

    let mut skip_comments = 0;
    if !val.comments.is_empty() {
        for comment in &val.comments {
            if comment.author.uuid != val.author.uuid {
                break;
            }

            skip_comments += 1;
            println!("\n{}", fill(&comment.body, &options));
        }
    }

    if !val.attachments.is_empty() {
        println!("\nAttachments:\n");
        for attachment in val.attachments {
            println!(
                "  - \"{}\" ({} bytes)",
                attachment.file_name, attachment.file_size
            );
            println!("    {}", attachment.url);
        }
    }

    if !val.likes.is_empty() {
        let likes = val
            .likes
            .iter()
            .map(|u| format!("{} (@{})", u.screen_name, u.username))
            .collect::<Vec<String>>()
            .join(", ");

        let options = Options::new(termwidth() - 2)
            .initial_indent("  ")
            .subsequent_indent("    ");

        let result = format!("{} liked this", likes);
        println!("\n{}", fill(&result, &options));
    }

    if val.comments.len() > skip_comments {
        println!("\nComments:\n");

        let options = Options::new(termwidth() - 2)
            .initial_indent("  ")
            .subsequent_indent("    ");

        let mut prev_author: Option<String> = None;
        for comment in &val.comments {
            if skip_comments > 0 {
                skip_comments -= 1;
                continue;
            }

            let curr_author = Some(String::from(&comment.author.uuid));

            let message = if prev_author == curr_author {
                format!("  {}\n", comment.body)
            } else {
                format!(
                    "{} (@{}):\n{}\n",
                    comment.author.screen_name, comment.author.username, comment.body
                )
            };

            prev_author = curr_author;
            println!("{}", fill(&message, &options));
        }
    }
}
