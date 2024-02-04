use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

pub struct ArgParser {
    /// Name of the person to greet
    #[arg(short = 'f', long = "file")]
    pub file: String,

    #[arg(short = 's', long = "search")]
    pub search: Option<String>,

    #[arg(short = 'b', long = "start_date")]
    start_date: Option<String>,

    #[arg(short = 'e', long = "end_date")]
    end_date: Option<String>,

    #[arg(short = 'q', long = "host")]
    host: Option<String>,

    #[arg(short = 'r', long = "request")]
    request: Option<String>,

    #[arg(short = 't', long = "http_status")]
    http_status: Option<String>,

    #[arg(short = 'o', long = "referer")]
    referer: Option<String>,

    #[arg(short = 'm', long = "large")]
    large: Option<u8>,

    #[arg(short = 'l', long = "lst")]
    last: Option<u8>,

    #[arg(short, long)]
    unique: Option<bool>,

    #[arg(short, long)]
    analytics: Option<bool>,

    #[arg(short = 'x', long = "sa")]
    session_analytics: Option<bool>,

    #[arg(short = 'i', long = "ip_ses")]
    ip_session: Option<String>,

    #[arg(short = 'c', long = "threads")]
    threads: Option<u8>,
}
