use std::io::Read;
const OPTION_STRING: &str = "i:p:o:";
fn create_app<'a>() -> clap::App<'a> {
    clap::App::new("teracli")
        .arg(
            clap::Arg::new("input")
                .takes_value(true)
                .long("input")
                .short('i')
                .required(true)
        )
        .arg(
            clap::Arg::new("parameter_file")
                .long("parameter")
                .short('p')
                .required(true)
        )
}

fn create_tera_context_from_parameter(json_path: &str) -> anyhow::Result<tera::Context> {
    let mut context = tera::Context::new();
    let f = std::fs::File::open(json_path)?;
    let parsed: serde_json::Value = serde_json::from_reader(f)?;
    match parsed {
        serde_json::Value::Object(v) => {

        }
    };
    Ok(context)
}

fn create_tera_context_from_jsonvalue(value: &serde_json::Value) -> tera::Context {

}

fn main() -> anyhow::Result<()> {
    let app = create_app();
    let matches = app.get_matches();
    let input_file = matches.value_of("input").unwrap();
    let parameter_file = matches.value_of("parameter_file").unwrap();

    Ok(())
}
