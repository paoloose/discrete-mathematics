import { useEffect, useState } from 'react';
import { DebounceInput } from 'react-debounce-input';
import QueryResultGraph from './QueryResultGraph';
import { vennfetch } from './fetching';


function QuerySection() {
  const [query, setQuery] = useState('id:*');

  useEffect(() => {
    vennfetch(`/api/records/?query=${query}`)
      .then(res => res.json())
      .then(records => {
        console.log(records);
      });
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
      <QueryResultGraph />
    </>
  );
}

export default QuerySection;
