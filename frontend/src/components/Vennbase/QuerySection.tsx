import { useEffect, useRef, useState } from 'react';
import { DebounceInput } from 'react-debounce-input';
import QueryResultGraph from './QueryResultGraph';
import { vennfetch } from './fetching';
import DataTable from './DataTable';
import type { LogicParsingResult, QueriedRecordResult } from '@types';

function QuerySection() {
  const [query, setQuery] = useState('id:*');
  const [queryResult, setQueryResult] = useState<QueriedRecordResult[]>([]);
  const [errorMsg, setErrorMsg] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const inputBoxRef = useRef<HTMLDivElement>(null);
  const [parsedInput, setParsedInput] = useState('<b class="token">true</b>');

  useEffect(() => {
    if (!query) {
      setQueryResult([]);
    }
    const controller = new AbortController();
    vennfetch(`/api/records/?query=${query}`, { abortSignal: controller.signal })
      .then(res => res.json())
      .then((records: QueriedRecordResult[]) => {
        setQueryResult(records);
      })
      .catch(() => {
        setQueryResult([]);
      });

    return () => {
      controller.abort();
    }
  }, [query]);

  const handleInput = async (input: string) => {
    if (!inputRef.current) return;
    const expression = input;
    const { parse_expression } = await import('logic-parsers');

    const parsed = JSON.parse(parse_expression(expression)) as LogicParsingResult;

    if (parsed.status === 'success') {
      // const { identifiers } = analizeTree(parsed.ast);
      // availableVariables.current = identifiers.reverse().map(iden => iden.name);
      setErrorMsg('');
      setParsedInput(
        input.split('').map(c => `<b class="token ${c === ' ' && 'space'}">${c}</b>`).join('')
      );
      console.log({ parsedInput })
    }
    else {
      // setAST(null);
      setErrorMsg(parsed.error);
      const { span: errorSpan } = parsed;
      setParsedInput(input.split('').map((c, i) => {
        if (i >= errorSpan[0] && i < errorSpan[1]) {
          return `<b class="token error ${c === ' ' && 'space'}">${c}</b>`;
        }
        return `<b class="token ${c === ' ' && 'space'}">${c}</b>`;
      }).join(''));
    }
  };

  const inputHandler = (e: Event) => {
    const target = e?.currentTarget as HTMLInputElement;
    handleInput(target.value);
  }

  useEffect(() => {
    if (!inputRef.current) return;
    inputRef.current.addEventListener('input', inputHandler);
    return () => {
      inputRef.current?.removeEventListener('input', inputHandler);
    };
  }, [inputRef]);

  return (
    <>
      <section id="query-form">
        <div
          id="rendered-input"
          ref={inputBoxRef}
        >
          <div dangerouslySetInnerHTML={{ __html: parsedInput }} />
          <div id="error-msg">{errorMsg}</div>
        </div>
        <DebounceInput
          autoCorrect="off"
          spellCheck={false}
          inputRef={inputRef}
          placeholder="write => here"
          debounceTimeout={200}
          type="text"
          onChange={e => setQuery(e.target.value)}
        />
      </section>
      <DataTable records={queryResult} />
      <div style={{ textAlign: "center", marginTop: "10px"}}>
        Se encontraron {queryResult.length} resultado(s)
      </div>
      {/* <QueryResultGraph queryResult={queryResult} /> */}
    </>
  );
}

export default QuerySection;
