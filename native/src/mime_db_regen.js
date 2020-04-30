/**
 * Usage:
 * node index.js > mime_types.rs
 */
const types = require("./db.json");
const extensions = new Map();

for (let mime in types) {
  if (!types[mime].extensions) {
    continue;
  }

  types[mime].extensions.forEach((extension) => {
    if (extensions.get(extension)) {
      if (types[mime].source === "iana") {
        extensions.set(extension, mime);
        return;
      }

      return;
    }

    extensions.set(extension, mime);
  });
}

console.log("use maplit::hashmap;");
console.log("use std::collections::HashMap;\n");
console.log("pub fn create_mime_map() -> HashMap<&'static str, &'static str> {");
console.log("    return hashmap!{");
Array.from(extensions).forEach(([ext, mime]) => {
    console.log(`        "${ext}" => "${mime}",`)
});
console.log("    };");
console.log("}");

