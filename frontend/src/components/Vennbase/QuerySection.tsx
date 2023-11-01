import { useEffect, useState } from 'react';
import { DebounceInput } from 'react-debounce-input';
import QueryResultGraph from './QueryResultGraph';
import { vennfetch } from './fetching';

export interface QueriedRecordResult {
  uuid: string;
  tags: string[];
  mimetype: string;
}

function QuerySection() {
  const [query, setQuery] = useState('id:*');
  const [queryResult, setQueryResult] = useState<QueriedRecordResult[]>([]);

  useEffect(() => {
    vennfetch(`/api/records/?query=${query}`)
      .then(res => res.json())
      .then((records: QueriedRecordResult[]) => {
        console.log(records);
        setQueryResult(records);
      })
      .catch(() => {});
  }, [query]);

  return (
    <>
      <section id="query-form">
        <DebounceInput
          autoCorrect="off"
          spellCheck={false}
          placeholder="write => here"
          debounceTimeout={300}
          type="text"
          onChange={e => setQuery(e.target.value)}
        />
      </section>
      <QueryResultGraph queryResult={queryResult} />
    </>
  );
}

export default QuerySection;
