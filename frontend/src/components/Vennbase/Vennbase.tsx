import type { RecordInformation } from '@types';
import { useEffect, useState } from 'react';
import DataTable from './DataTable';
import { vennfetch } from './fetching';
import QuerySection from './QuerySection';

function Vennbase() {
  const [records, setRecords] = useState<RecordInformation[]>([]);

  useEffect(() => {
    vennfetch('/api/records/')
      .then(res => res.json())
      .then(records => {
        console.log(records);
        setRecords(records);
      });
  }, []);

  return (
    <article id="vennbase">
      <h1>Vennbase</h1>
      <DataTable records={records} />
      <QuerySection />
    </article>
  );
}

export default Vennbase;
