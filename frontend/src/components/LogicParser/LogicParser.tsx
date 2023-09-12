import { useEffect, useRef, useState } from 'preact/hooks';
import SVGRender from '@components/SVGRender';
import type { TargetedEvent } from 'preact/compat';
import type { LogicParsingResult } from '@types';
import { analizeTree } from './analize';
import { tokenizeExpr } from './tokenize';

function LogicParser() {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const [svgStr, setSvgStr] = useState('');
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const inputBoxRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!inputBoxRef.current) return;
    handleInput();
    ['click', 'touch'].forEach(event => {
      inputBoxRef.current?.addEventListener(event, (e) => {
        inputRef.current?.focus();
      });
    });
  }, []);

  const handleInput = async (_?: TargetedEvent<HTMLTextAreaElement, Event>) => {
    if (!inputRef.current) return;
    const expression = inputRef.current.value;
    const { parse_expression, generate_svg } = await import('logic-parsers');

    setInput(expression)
    const parsed = JSON.parse(parse_expression(expression)) as LogicParsingResult;
    const formattedOutput = JSON.stringify(parsed, null, 4);

    const tokens = tokenizeExpr(expression);

    if (parsed.status === 'success') {
      const renderedSvg = generate_svg(parsed.ast, 20, 30, 10);
      setSvgStr(renderedSvg);
      analizeTree(parsed.ast);
    }
    setOutput(formattedOutput);
  };

  return (
    <section id="logic-parser">
      <aside>
        <div id="input-wrapper">
          <div ref={inputBoxRef}>
            {input}
          </div>
          <textarea
            type="text"
            ref={inputRef}
            value={input}
            onInput={handleInput}
          />
        </div>
      </aside>
      <pre id="output">
        {output}
      </pre>
      {
        svgStr ? <SVGRender svgStr={svgStr} /> : null
      }
    </section>
  );
}

export default LogicParser;
