# MimeDetect

> MIME-type detector

<br>

**MimeDetect** is an experiment on writing a Node-module with Rust (using [neon-bindings](https://github.com/neon-bindings/neon)) and wrapping everything up into an [Electron](https://www.electronjs.org/) application.

Usage is pretty straightforward: you drop a file — you get its MIME-type. You can also drop multiple files or even a folder.

**MimeDetect** under the hood uses the [infer](https://github.com/bojand/infer) module to check file’s MIME-type by one's byte signature. If the `infer` fails, the module falls back to checking MIME-type by file’s extension against [mime-db](https://github.com/jshttp/mime-db).

<br>

## Screenshot

![Screenshot at May 19 19-45-40](https://user-images.githubusercontent.com/24318966/82354391-69490e00-9a09-11ea-9766-a2454504efc0.png)

## Building

To set everything up, follow Neon’s [official guide](https://neon-bindings.com/docs/getting-started/).

After that:

1. [Clone the repo](https://github.com/limitedeternity/MimeDetect/archive/master.zip)

2. Run: `npm install && npm run package`

This command will produce a ready-to-use application.

## Meta

Distributed under the GPL-3.0 license. See ``LICENSE`` for more information.

[@limitedeternity](https://github.com/limitedeternity)
