import type { RecordInformation } from '@types';
import { useEffect, useState } from 'preact/hooks';

const BACKEND_ADDR = 'http://127.0.0.1:8000'

async function vennfetch(endpoint: string, method: string = 'GET') {
  const response = await fetch(`${BACKEND_ADDR}${endpoint}`, {
    method
  });
  return response;
}

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
      <table id="records-list">
        <thead>
          <tr>
            <th class="record-repr"></th>
            <th class="record-info-mimetype">Media Type</th>
            <th class="record-info-name">Nombre</th>
          </tr>
        </thead>
        <tbody>
        {
          records.length > 0 && records.map(record => (
            <tr key={record.id}>
              <td class="record-repr">
                <img
                  src={`${BACKEND_ADDR}/api/records/${record.vennbase_id}?resize=autox30`}
                />
              </td>
              <td class="record-info-mimetype">{record.mimetype}</td>
              <td class="record-info-name">{record.name}</td>
            </tr>
          ))
        }
        </tbody>
      </table>
    </article>
  );
}

export default Vennbase;
