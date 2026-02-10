use dprint_plugin_asciidoc::configuration::Configuration;
use schemars::schema_for;
use std::io::stdout;

fn main() {
    let schema = schema_for!(Configuration);
    serde_json::to_writer_pretty(stdout(), &schema).unwrap();
}
