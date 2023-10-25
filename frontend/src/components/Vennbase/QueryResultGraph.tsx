import UpSetJS, { extractCombinations } from '@upsetjs/react';
// import the upsetjs react component

function QueryResultGraph() {

  const elems = [
    { name: 'A', sets: ['S1', 'S2'] },
    { name: 'B', sets: ['S1'] },
    { name: 'C', sets: ['S2'] },
    { name: 'D', sets: ['S1', 'S3'] },
  ];

  const { sets, combinations } = extractCombinations(elems);
  const UpSet = UpSetJS as any;

  return (
    <section id="query-result-graph">
      <UpSet sets={sets} combinations={combinations} width={500} height={300} />
    </section>
  );
}

export default QueryResultGraph;
