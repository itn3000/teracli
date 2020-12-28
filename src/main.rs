use std::io::{ Write, Read };

use clap::ArgMatches;

fn create_app<'a>() -> clap::App<'a> {
    clap::App::new("teracli")
        .arg(
            clap::Arg::new("input")
                .value_name("INPUT_FILE")
                .long("input")
                .short('i')
        )
        .arg(
            clap::Arg::new("parameter_file")
                .long("parameter")
                .short('p')
                .value_name("PARAMETER_JSON")
                .required(true)
        )
        .arg(
            clap::Arg::new("output_file")
                .long("output")
                .short('o')
                .value_name("OUTPUT_FILE")
        )
}

fn create_tera_context_from_parameter(json_path: &str) -> anyhow::Result<tera::Context> {
    let f = std::fs::File::open(json_path)?;
    let parsed: serde_json::Value = serde_json::from_reader(f)?;
    // let context = create_tera_context_from_jsonvalue(&parsed);
    let context = tera::Context::from_serialize(parsed)?;
    Ok(context)
}

fn get_input_from_matches(matches: &clap::ArgMatches) -> anyhow::Result<String> {
    Ok(match matches.value_of("input") {
        Some(v) => {
            let mut fi = std::fs::File::open(v)?;
            let mut ret = String::new();
            fi.read_to_string(&mut ret)?;
            ret
        },
        None => {
            let mut si = std::io::stdin();
            let mut ret = String::new();
            si.read_to_string(&mut ret)?;
            ret
        }
    })
}

fn main() -> anyhow::Result<()> {
    let app = create_app();
    let matches = app.get_matches();
    let parameter_file = matches.value_of("parameter_file").unwrap();
    let context = create_tera_context_from_parameter(parameter_file)?;
    let mut tr = tera::Tera::default();
    let input_string = get_input_from_matches(&matches)?;
    let parsed = tr.render_str(&input_string, &context)?;
    match matches.value_of("output") {
        Some(v) => {
            let mut o = std::fs::File::create(v)?;
            let bytes = parsed.as_bytes();
            o.write(&bytes)?;
        },
        None => {
            let mut so = std::io::stdout();
            let bytes = parsed.as_bytes();
            so.write(&bytes)?;
        }
    };
    Ok(())
}
