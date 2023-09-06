# Wasm bindings for the logic-parser

Wasm bindings for the `../logic-parser` written in Rust.

Packaging and publishing was achieved thanks to `wasm-bindgen`!

```console
npm install logic-parsers
```

## Usage

```ts
const { parse_expression, generate_svg } = await import('logic-parsers');

const parsed = parse_expression("((p || q)) => (q && ~(r))");
```

Outputs:

```json
{
    "status": "success",
    "ast": {
        "type": "operator.implies",
        "left": {
            "type": "operator.or",
            "left": {
                "type": "identifier",
                "name": "p"
            },
            "right": {
                "type": "identifier",
                "name": "q"
            }
        },
        "right": {
            "type": "operator.and",
            "left": {
                "type": "identifier",
                "name": "q"
            },
            "right": {
                "type": "operator.not",
                "operand": {
                    "type": "identifier",
                    "name": "r"
                }
            }
        }
    }
}
```

SVG tree rendering:

```ts
const svg_xml = generate_svg(parsed.ast);
document.querySelector("#output").innerHTML = svg_xml;
```

![SVG Result](../logic-parser/assets/resulting_tree.png)
