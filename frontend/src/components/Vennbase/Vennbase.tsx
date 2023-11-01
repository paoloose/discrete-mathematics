import type { RecordInformation } from '@types';
import { useEffect, useState } from 'react';
import DataTable from './DataTable';
import { vennfetch } from './fetching';
import QuerySection from './QuerySection';
import VennbaseStatus from './VennbaseStatus';

function Vennbase() {
  const [records, setRecords] = useState<RecordInformation[]>([]);

  useEffect(() => {
    vennfetch('/api/records/')
      .then(res => res.json())
      .then(records => {
        console.log(records);
        setRecords(records);
      })
      .catch(() => {});
  }, []);

  return (
    <article id="vennbase">
      <h1>Vennbase</h1>
      <VennbaseStatus />
      <DataTable records={records} />
      <QuerySection />
    </article>
  );
}

export default Vennbase;
