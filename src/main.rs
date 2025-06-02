use std::{net::TcpStream, path::Path, process::Command, time::Duration};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "AppRunnerAndWaiter",
    about = "Runs an app and wait's for it's port to open."
)]
struct Args {
    #[arg(
        short,
        long,
        help = "The working directory to run the app in. Default is the parent of --app"
    )]
    directory: Option<String>,

    #[arg(short, long, help = "The file to run, e.g. executable, .sh")]
    app: String,

    #[arg(short, long, help = "The port to wait for, then open.")]
    port: u16,

    #[arg(last = true, help = "The arguments to provide to the --app")]
    app_args: Vec<String>,

    #[arg(
        short,
        long,
        default_value = "xdg-open",
        help = "How it should open the finished URL"
    )]
    opener: String,

    #[arg(
        short,
        long,
        default_value = "30",
        help = "How long it should wait for the application should open (seconds)"
    )]
    timeout: u8,
}

fn main() {
    let args = Args::parse();

    let app = Path::new(&args.app);
    if !app.exists() {
        eprintln!("Provided --app does not exist!");
        return;
    }

    if !app.is_file() {
        eprintln!("Provided --api must be a file!");
        return;
    }

    let directory = match &args.directory {
        Some(dir) => Path::new(dir),
        None => app.parent().unwrap(),
    };

    if !directory.exists() {
        eprintln!("Provided --directory does not exist!");
        return;
    }

    let mut command = Command::new(&args.app)
        .current_dir(directory)
        .args(args.app_args)
        .spawn()
        .unwrap_or_else(|e| panic!("Failed to spawn process: {}", e));

    for _ in 0..args.timeout {
        if is_port_open("127.0.0.1", args.port) {
            Command::new(&args.opener)
                .arg(format!("http://127.0.0.1:{}", args.port))
                .spawn()
                .unwrap_or_else(|e| panic!("Failed to open URL via {}: {}", args.opener, e));

            println!("Sucessfully started {}!", args.app);
            command
                .wait()
                .unwrap_or_else(|e| panic!("Failed to wait for app to close: {}", e));

            return;
        }
    }

    eprintln!("The application took too long to start. Change this with --timeout");
}

fn is_port_open(host: &str, port: u16) -> bool {
    let address = format!("{}:{}", host, port);
    TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(1)).is_ok()
}
