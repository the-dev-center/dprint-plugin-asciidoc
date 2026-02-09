/**
 * TypeScript type definitions for dprint-plugin-asciidoc
 * 
 * This file provides TypeScript support for the npm package.
 */

declare module 'dprint-plugin-asciidoc' {
  /** Path to the WASM plugin file */
  export const path: string;
  
  /** Package name */
  export const name: string;
  
  /** Package version */
  export const version: string;
  
  /** Configuration key used in dprint.jsonc */
  export const configKey: string;
}