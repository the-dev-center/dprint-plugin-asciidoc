/**
 * dprint-plugin-asciidoc - AsciiDoc formatter plugin for dprint
 * 
 * This file is the entry point for the npm package. It provides the path
 * to the WASM plugin for dprint to consume.
 */

const path = require('path');

// Export the path to the WASM file
module.exports = {
  // Path to the WASM plugin
  path: path.join(__dirname, 'dprint_plugin_asciidoc.wasm'),
  
  // Plugin metadata
  name: 'dprint-plugin-asciidoc',
  version: require('./package.json').version,
  
  // Configuration key used in dprint.jsonc
  configKey: 'asciidoc'
};