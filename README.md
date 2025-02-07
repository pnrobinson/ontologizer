# Ontologizer

This repository is intended to try features of tauri and SvelteKit.
It was initialized from the [tauri sveltekit-admin tempalte](https://github.com/deid84/tauri-sveltekit-admin-template),
 a starter template for building Tauri apps using SvelteKit with Tailwind CSS.


## Prerequisites

Before you can use this template, you will need to install the following dependencies:

-   [Node.js](https://nodejs.org/)
-   [Rust & OS Dev Tools](https://tauri.app/v1/guides/getting-started/prerequisites)

All the other dependencies are already in `package.json` so they will be automatically installed during next step.

## Getting started

1.  Change into the `target-directory`:

```
cd <target-directory>

```

2.  Install the dependencies:

```
npm install

```

3.  Start the development server:

```
npm run tauri dev

```

or start dev server for browser

```
npm run dev

```

This will start the development server and open a new window. The app will automatically reload whenever you make changes to the source code.

## Building for production

To build the app for production, run the following command:

```
npm run build

```

This will create a production build of the app in the `build` directory.


# Ontologizer

See [Bauer et al., 2008](https://pubmed.ncbi.nlm.nih.gov/18511468/) for details on the ontologizer. We will be adding functionality gradually.

For now, download the GO annotation file for Homo sapiens from the [GAF download page](https://current.geneontology.org/products/pages/downloads.html). Start the GUI, and open this file.