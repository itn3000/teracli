use std::io::{ Write, Read };
use clap::{crate_name, crate_version};

fn create_app<'a>() -> clap::App<'a> {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .arg(
            clap::Arg::new("input")
                .value_name("INPUT_FILE")
                .long("input")
                .short('i')
        )
        .arg(
            clap::Arg::new("parameter_file")
                .long("json-parameter")
                .short('j')
                .value_name("PARAMETER_JSON")
        )
        .arg(
            clap::Arg::new("key_parameter")
                .long("key-parameter")
                .short('k')
                .multiple_occurrences(true)
                .help("key-value pair devided by '=', complex type did not allowed")
                .value_name("KEYVALUE_PAIR")
        )
        .arg(
            clap::Arg::new("output_file")
                .long("output")
                .short('o')
                .value_name("OUTPUT_FILE")
        )
}

fn create_tera_context_from_parameter_json(json_path: &str) -> anyhow::Result<tera::Context> {
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

fn create_tera_context_from_keyvalue(kvlist: &[&str]) -> anyhow::Result<tera::Context> {
    let mut context = tera::Context::default();
    for kv in kvlist.iter() {
        let pair: Vec<&str> = (*kv).splitn(2, "=").collect();
        if pair.len() < 2 {
            continue;
        }
        context.insert(pair[0], pair[1]);
    }
    Ok(context)

}

fn main() -> anyhow::Result<()> {
    let app = create_app();
    let matches = app.get_matches();
    let parameter_file = matches.value_of("parameter_file").unwrap();
    let mut context = create_tera_context_from_parameter_json(parameter_file)?;
    match matches.values_of("key_parameter") {
        Some(v) => {
            let keyvalues: Vec<&str> = v.collect();
            let kv_context = create_tera_context_from_keyvalue(&keyvalues)?;
            context.extend(kv_context);
        },
        None => ()
    };
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
