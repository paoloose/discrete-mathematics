import { useEffect, useRef, useState } from 'preact/hooks';
import SVGRender from '@components/SVGRender';
import type { TargetedEvent } from 'preact/compat';
import type { ASTNode, LogicParsingResult } from '@types';
import { analizeTree } from './analize';
import { tokenizeExpr } from './tokenize';

function LogicParser() {
  const [input, setInput] = useState('p => q');
  const [output, setOutput] = useState('');
  const [ast, setAST] = useState<ASTNode | null>(null);
  const parsedInput = useRef('');
  const inputRef = useRef<HTMLInputElement>(null);
  const inputBoxRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    handleInput();
  }, []);

  const handleInput = async (_?: TargetedEvent<HTMLInputElement, Event>) => {
    if (!inputRef.current) return;
    const expression = inputRef.current.value;
    const { parse_expression } = await import('logic-parsers');

    setInput(expression);
    const parsed = JSON.parse(parse_expression(expression)) as LogicParsingResult;

    try {
      parsedInput.current = tokenizeExpr(expression).map(token => {
        const repr = expression.slice(token.span[0], token.span[1]);
        return `<b class="token">${repr}</b>`;
      }).join('');
    } catch (e) {
    }

    if (parsed.status === 'success') {
      analizeTree(parsed.ast);
      setAST(parsed.ast);
      setOutput(JSON.stringify(parsed.ast, null, 4));
    }
    else {
      setAST(null);
    }
  };

  return (
    <section id="logic-parser">
      <div id="input-wrapper">
        <div
          id="rendered-input"
          ref={inputBoxRef}
        >
          <div dangerouslySetInnerHTML={{ __html: parsedInput.current }}>
          </div>
        </div>
        <input
          autocorrect="off"
          spellCheck={false}
          type="text"
          ref={inputRef}
          value={input}
          onInput={handleInput}
        />
      </div>
      { ast && <SVGRender ast={ast} /> }
      <pre id="output">
        {output}
      </pre>
    </section>
  );
}

export default LogicParser;
