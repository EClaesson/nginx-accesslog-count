use clap::{Parser, ValueEnum};
use std::fmt;
use std::fmt::Formatter;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "snake_case")]
pub enum Column {
    Address,
    User,
    Time,
    Request,
    Status,
    BytesSent,
    Referer,
    UserAgent,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let formatted = match self {
            Column::Address => "address",
            Column::User => "user",
            Column::Time => "time",
            Column::Request => "request",
            Column::Status => "status",
            Column::BytesSent => "bytes_sent",
            Column::Referer => "referer",
            Column::UserAgent => "user_agent",
        };

        write!(f, "{}", formatted)
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true, about = "Count frequencies of columns in nginx access logs.\nhttps://github.com/EClaesson/nginx-accesslog-count"
)]
pub struct Arguments {
    #[arg(short, long, required = true, help = "Column to count")]
    pub column: Column,

    #[arg(short, long, default_value = "desc", help = "Order to sort")]
    pub order: SortOrder,

    #[arg(
        short,
        long,
        default_value = "0",
        help = "Number of lines to show. 0 will show all lines"
    )]
    pub limit: u32,

    #[arg(short, long, action, help = "Suppress all output except result list")]
    pub quiet: bool,

    #[arg(short, long, action, help = "Show only column value without count")]
    pub no_count: bool,

    #[arg(
        short,
        long,
        default_value = "",
        conflicts_with = "whitelist",
        help = "Exclude lines where column matches regex pattern."
    )]
    pub exclude: String,

    #[arg(
        short,
        long,
        default_value = "",
        conflicts_with = "exclude",
        help = "Only include lines where column matches regex pattern."
    )]
    pub whitelist: String,

    #[arg(required = true, help = "Whitespace separated list of log files to read")]
    pub files: Vec<String>,
}