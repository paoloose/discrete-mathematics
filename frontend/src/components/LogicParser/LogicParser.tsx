import { useEffect, useRef, useState } from 'react';
import SVGRender from '@components/SVGRender';
import type { ASTNode, LogicParsingResult } from '@types';
import { analizeTree } from './analize';
import { generateTable } from './generateTable';

function LogicParser() {
  const [input, setInput] = useState('p => (q & r)');
  const [output, setOutput] = useState('');
  const [ast, setAST] = useState<ASTNode | null>(null);
  const parsedInput = useRef('');
  const inputRef = useRef<HTMLInputElement>(null);
  const inputBoxRef = useRef<HTMLDivElement>(null);
  const clickableTokens = useRef(['=>', '<=>', '&', '|', '!', '(', ')', 'true', 'false']);
  const availableVariables = useRef<string[]>([]);
  const [errorMsg, setErrorMsg] = useState('');

  const tabsViews = useRef(['Modo JSON', 'Modo árbol', 'Modo tabla']);
  const [currentView, setCurrentView] = useState(1);

  const handleInput = async () => {
    if (!inputRef.current) return;
    const expression = input;
    const { parse_expression } = await import('logic-parsers');

    const parsed = JSON.parse(parse_expression(expression)) as LogicParsingResult;

    if (parsed.status === 'success') {
      const { identifiers } = analizeTree(parsed.ast);
      availableVariables.current = identifiers.reverse().map(iden => iden.name);
      setAST(parsed.ast);
      setOutput(JSON.stringify(parsed.ast, null, 4));
      setErrorMsg('');
      parsedInput.current = input.split('').map(c => `<b class="token ${c === ' ' && 'space'}">${c}</b>`).join('');
    }
    else {
      setAST(null);
      setErrorMsg(parsed.error);
      const { span: errorSpan } = parsed;
      parsedInput.current = input.split('').map((c, i) => {
        if (i >= errorSpan[0] && i < errorSpan[1]) {
          return `<b class="token error ${c === ' ' && 'space'}">${c}</b>`;
        }
        return `<b class="token ${c === ' ' && 'space'}">${c}</b>`;
      }).join('');
    }
  };

  useEffect(() => {
    handleInput();
  }, [input]);

  return (
    <section id="logic-parser">
      <div id="input-wrapper">
        <div
          id="rendered-input"
          ref={inputBoxRef}
        >
          <div dangerouslySetInnerHTML={{ __html: parsedInput.current }} />
          <div id="error-msg">{errorMsg}</div>
        </div>
        <input
          autoCorrect="off"
          spellCheck={false}
          placeholder="write => here"
          type="text"
          ref={inputRef}
          value={input}
          onInput={e => setInput(e.currentTarget.value)}
        />
      </div>
      <section id="clickable-tokens">
        {
          clickableTokens.current.concat(availableVariables.current).map((token) => (
            <button
              key={token}
              onClick={() => {
                if (!inputRef.current) return;
                const { selectionStart, selectionEnd } = inputRef.current;
                const newInput = `${input.slice(0, selectionStart!)}${token}${input.slice(selectionEnd!)}`;
                setInput(newInput);
                inputRef.current.focus();
                setTimeout(() => {
                  inputRef.current!.setSelectionRange(
                    selectionStart! + token.length,
                    selectionStart! + token.length
                  );
                }, 0);
                handleInput();
              }}
            >
              {token}
            </button>
          ))
        }
      </section>

      <section id="tabs-buttons">
        {
          tabsViews.current.map((tab, i) => (
            <button
              key={tab}
              className={currentView === i ? 'active' : ''}
              onClick={() => setCurrentView(i)}
            >
              {tab}
            </button>
          ))
        }
      </section>

      <section>
        {
          currentView === 0 && <pre id="output">{output}</pre>
        }
        {
          currentView === 1 && ast && <SVGRender ast={ast} />
        }
        {
          currentView === 2 && ast && <table>
            { generateTable(ast).map((row, i) => (
              i === 0
                ? <tr key={i}>{row.map((cell, j) => <th key={j}>{cell}</th>)}</tr>
                : <tr key={i}>{row.map((cell, j) => <td key={j}>{cell}</td>)}</tr>
            )) }
          </table>
        }
      </section>
    </section>
  );
}

export default LogicParser;
