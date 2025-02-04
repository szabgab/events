# Virtual Events


* See the `rust.yaml` file for content.

## Generate the site

```
cargo run
```

This will create the `_site` folder.

## View site locally

Install [rustatic](https://rustatic.code-maven.com/)

and after generating the static pages with the previously described command, run

```
rustatic --host localhost --port 5000 --indexfile index.html --nice --path _site/
```

