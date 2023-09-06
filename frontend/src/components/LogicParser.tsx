import { useEffect, useRef, useState } from 'react';
import type { FormEvent } from 'react';
import SVGRender from '@components/SVGRender';

function LogicParser() {
  const [output, setOutput] = useState('');
  const [svgStr, setSvgStr] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
  }, []);

  const handleInput = async (_: FormEvent<HTMLInputElement>) => {
    if (!inputRef.current) return;

    const { parse_expression, generate_svg } = await import('logic-parsers');

    const parsed = JSON.parse(parse_expression(inputRef.current.value));
    const formattedOutput = JSON.stringify(parsed, null, 4);

    if (parsed.status === 'success') {
      const renderedSvg = generate_svg(parsed.ast, 20, 30, 10);
      setSvgStr(renderedSvg);
    }
    setOutput(formattedOutput);
  };

  return (
    <main>
      <aside>
        <input type="text" id="input" ref={inputRef} onChange={handleInput} />
      </aside>
      <div>
        <pre id="output">
          {output}
        </pre>
        {
          svgStr ? <SVGRender svgStr={svgStr} /> : null
        }
      </div>
    </main>
  );
}

export default LogicParser;
