import ForceGraph from 'force-graph';

const SERVER_ENDPOINT = '127.0.0.1:3030';
const WS_SERVER_URL = `ws://${SERVER_ENDPOINT}/ws`;

const ws = new WebSocket(WS_SERVER_URL);
const root = document.getElementById('graph')!;

ws.addEventListener('open', () => {
  console.log('ws: connected');
});

ws.addEventListener('message', (m) => {
  console.log({m});
});

const N = 300;
const gData = {
  nodes: [...Array(N).keys()].map(i => ({ id: i })),
  links: [...Array(N).keys()]
    .filter(id => id)
    .map(id => ({
      source: id,
      target: Math.round(Math.random() * (id-1))
    }))
};

const graph = ForceGraph()(root)
  .backgroundColor('#0b0b0b')
  .linkDirectionalParticles(2)
  .graphData(gData);

window.addEventListener('resize', () => {
  graph.width(window.innerWidth)
  graph.height(window.innerHeight + 4)
});
