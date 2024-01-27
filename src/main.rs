use clap::{arg, command, value_parser, Arg, ArgAction};
use preprocess_cancellation::Slicer;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("paths")
                .action(ArgAction::Append)
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output-suffix")
                .long("output-suffix")
                .short('o')
                .required(false)
                .help(
                    "Add a suffix to gcoode output. Without this, gcode will be rewritten in place",
                ),
        )
        .arg(
            Arg::new("disable-shapely")
                .long("disable-shapely")
                .help("Deprecated, has no effect")
                .hide(true)
                .required(false),
        )
        .get_matches();

    let output_suffix = matches.get_one::<String>("output-suffix");
    let paths = matches.get_many::<Vec<PathBuf>>("paths").unwrap();

    for gcode_file in paths {
        let mut file = std::fs::File::open(&gcode_file).unwrap();
        let mut objects = preprocess_cancellation::list_objects(&mut file).unwrap();
        let mut dst = if let Some(output_suffix) = output_suffix {
            let mut dst = gcode_file.clone();
            dst.set_extension(output_suffix);
            std::fs::File::create(dst).unwrap()
        } else {
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file
        };
        preprocess_cancellation::rewrite_to(BufReader::new(file), &mut objects, &mut dst).unwrap();
    }
}
