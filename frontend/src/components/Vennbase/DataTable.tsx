import type { RecordInformation } from '@types';
import { BACKEND_ADDR } from './fetching';

const VISUALIZABLE_MIMETYPES = [
  'image/jpeg',
  'image/png',
  'image/gif',
  'image/webp',
  'image/svg+xml',
  'image/x-icon',
  'audio/ogg',
  'audio/wav',
  'audio/webm',
];

function DataTable({ records }: { records: RecordInformation[] }) {

  return (
    <table id="records-list">
      <thead>
        <tr>
          <th className="record-repr"></th>
          <th className="record-info-mimetype">Media Type</th>
          <th className="record-info-name">Nombre</th>
        </tr>
      </thead>
      <tbody>
      {
        records.length > 0 && records.map(record => (
          <tr key={record.id}>
            <td className="record-repr">
              {
                VISUALIZABLE_MIMETYPES.includes(record.mimetype)
                  ? <img
                    src={`${BACKEND_ADDR}/api/records/${record.vennbase_id}?resize=autox30`}
                    loading="lazy"
                  />
                  : <img
                    src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/97/Document_icon_%28the_Noun_Project_27904%29.svg/1024px-Document_icon_%28the_Noun_Project_27904%29.svg.png"
                    height="30"
                    width="24"
                  />
              }
            </td>
            <td className="record-info-mimetype">{record.mimetype}</td>
            <td className="record-info-name">{record.name}</td>
          </tr>
        ))
      }
      </tbody>
    </table>
  );
}

export default DataTable;
