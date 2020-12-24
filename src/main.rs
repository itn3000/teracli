use std::io::Read;
use serde_json::Value;
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

fn create_tera_context_from_jsonvalue(value: &Value) -> tera::Context {
    let context = tera::Context::new();
    context.extend()
    match value {
        Value::Object(v) => {
            for k in v.keys() {
                match v[k] {
                    Value::Object(inner) => 
                        context.insert(k, &create_tera_context_from_jsonvalue(&Value::Object(inner))),
                    Value::String(inner) => context.insert(k, &inner),
                    Value::Number(inner) => context.insert(k, &inner),
                    Value::Array(inner) => context.insert(k, &inner),
                    Value::Bool(inner) => context.insert(k, &inner),
                    Value::Null => (),
                };
            }
        },
        _ 
    };
    context
}

fn main() -> anyhow::Result<()> {
    let app = create_app();
    let matches = app.get_matches();
    let input_file = matches.value_of("input").unwrap();
    let parameter_file = matches.value_of("parameter_file").unwrap();

    Ok(())
}
