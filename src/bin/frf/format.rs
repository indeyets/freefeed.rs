use textwrap::{Wrapper, termwidth};
use freefeed::api::Post;

pub fn format_post(val: Post) {
    let w = Wrapper::with_termwidth()
        .initial_indent("")
        .subsequent_indent("     ");

    let from = format!("{} (@{})", val.author.screen_name, val.author.username);
    println!("{}", w.fill(&from));

    let date = if val.created_at == val.updated_at {
        format!("on {}", val.created_at)
    } else {
        format!("on {} (updated on {})", val.created_at, val.updated_at)
    };

    println!("{}", w.fill(&date));

    let w = Wrapper::new(termwidth() - 2)
        .initial_indent("    ")
        .subsequent_indent("    ");
    println!("\n{}", w.fill(&val.body));

    let mut skip_comments = 0;
    if val.comments.len() > 0 {
        for comment in &val.comments {
            if comment.author.uuid != val.author.uuid {
                break;
            }

            skip_comments += 1;
            println!("\n{}", w.fill(&comment.body));
        }
    }

    if val.attachments.len() > 0 {
        println!("\nAttachments:\n");
        for attachment in val.attachments {
            println!("  - \"{}\" ({} bytes)", attachment.file_name, attachment.file_size);
            println!("    {}", attachment.url);
        }
    }

    if val.likes.len() > 0 {
        let likes = val.likes.iter().map(|u| format!("{} (@{})", u.screen_name, u.username)).collect::<Vec<String>>().join(", ");

        let w = Wrapper::new(termwidth() - 2)
            .initial_indent("  ")
            .subsequent_indent("    ");

        let result = format!("{} liked this", likes);
        println!("\n{}", w.fill(&result));
    }

    if val.comments.len() > skip_comments {
        println!("\nComments:\n");

        let w = Wrapper::new(termwidth() - 2)
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
                format!("{} (@{}):\n{}\n", comment.author.screen_name, comment.author.username, comment.body)
            };

            prev_author = curr_author;
            println!("{}", w.fill(&message));
        }
    }
}
