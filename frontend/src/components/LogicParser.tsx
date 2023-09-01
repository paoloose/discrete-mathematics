import { useEffect, useRef, useState } from 'react';
import type { FormEvent } from 'react';

function LogicParser() {
  const [output, setOutput] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
  }, []);

  const handleInput = async (_: FormEvent<HTMLInputElement>) => {
    if (!inputRef.current) return;

    const { parse_expression } = await import('logic-parsers');
    const parsed = parse_expression(inputRef.current.value);
    const formattedOutput = JSON.stringify(JSON.parse(parsed), null, 4);
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
      </div>
    </main>
  );
}

export default LogicParser;
