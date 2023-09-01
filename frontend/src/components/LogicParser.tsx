import { parse_expression } from 'logic-parsers';
import { FormEvent, useEffect, useRef, useState } from 'react';

function Thing() {
  const [output, setOutput] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
  }, []);

  const handleInput = (e: FormEvent<HTMLInputElement>) => {
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

export default Thing;
