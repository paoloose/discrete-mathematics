# Logic syntax tree generator

Rust library for lexing, parsing and visualizing logical expressions.

The code has no relevant dependencies more than `thiserror` for improving the
error handling and an **optional** `serde` feature for deserializing a syntax
tree from a well-formed JSON input.

The parser currently process one-liner expressions with the following
[Backus-Naur](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) form:

```txt
expr := term [(<-> | ->) expr]
term := prop [(|| | &&) term]
prop := [~] ("true" | "false" | name | LPAREN expr RPAREN)
```

The concept is very new for me so don't expect this syntax to comply the
standard. 😁 The parser is currently working as expected, reporting meaningful
syntax errors and parsing any complex expression.

Want to use this library for the Web? Web Assembly bindings were also written!
Look at the (unpublished) [`logic-parser-wasm`](https://github.com/paoloose/discrete-mathematics/tree/main/assignments/week1/logic-parser-wasm) library.

## Examples

```rs
let expression = "((p || q)) => (q && ~(r))";

let tokens: Vec<Token> = Lexer::new().tokenize(expression)?;
let ast: ASTNode = Parser::new(&tokens).parse()?;

ast.as_json()
```

JSON output:

```json
{
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
```

Rendered SVG tree:

```rs
let horizontal_separation = 20_f32;
let vertical_separation = 30_f32;
let radius = 15_f32;

// ((p || q)) => (q && ~(r))

let svg = render_to_svg(
    ast,
    horizontal_separation,
    vertical_separation,
    radius
);

svg.as_xml()
```

![Resulting tree](https://raw.githubusercontent.com/paoloose/discrete-mathematics/main/assignments/week1/logic-parser/assets/resulting_tree.png)

## Testing

Unit tests were written for all the relevant parts of the library.

```console
cargo test
```

## References

- <https://michael-f-bryan.github.io/static-analyser-in-rust/book/parse/parser.html>
