use std::io::stdout;
use schemars::schema_for;
use dprint_plugin_asciidoc::configuration::Configuration;

fn main() {
    let schema = schema_for!(Configuration);
    serde_json::to_writer_pretty(stdout(), &schema).unwrap();
}
