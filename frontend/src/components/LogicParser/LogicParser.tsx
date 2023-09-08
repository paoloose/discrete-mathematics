import { useEffect, useRef, useState } from 'preact/hooks';
import SVGRender from '@components/SVGRender';
import type { TargetedEvent } from 'preact/compat';

function LogicParser() {
  const [output, setOutput] = useState('');
  const [svgStr, setSvgStr] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
  }, []);

  const handleInput = async (_: TargetedEvent<HTMLInputElement, Event>) => {
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
    <>
      <aside>
        <input type="text" id="input" ref={inputRef} onKeyPress={handleInput} />
      </aside>
      <div>
        <pre id="output">
          {output}
        </pre>
        {
          svgStr ? <SVGRender svgStr={svgStr} /> : null
        }
      </div>
    </>
  );
}

export default LogicParser;
