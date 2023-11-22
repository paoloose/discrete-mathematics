import ForceGraph, { type GraphData, type LinkObject, type NodeObject } from 'force-graph';
import { isValidURL } from './utils';

const SERVER_ENDPOINT = '127.0.0.1:3030';
const NODE_R = 4;
const WS_SERVER_URL = `ws://${SERVER_ENDPOINT}/ws`;

const ws = new WebSocket(WS_SERVER_URL);
const $root = document.getElementById('graph') as HTMLDivElement;
const $input = document.querySelector('#search-box input') as HTMLInputElement;
const $currentLink = document.getElementById('current-link') as HTMLDivElement;
const $activeOrigins = document.getElementById('active-origins') as HTMLDivElement;

let activeOrigins: string[] = [];

ws.addEventListener('open', () => {
  console.log('ws: connected');
});

ws.addEventListener('close', () => {
  console.log('ws: disconnected');
});

ws.addEventListener('error', (e) => {
  console.log('ws: error', e);
});

const gData: GraphData = {
  nodes: [],
  links: []
};

// extend the NodeObject global Type
declare module 'force-graph' {
  interface NodeObject {
    links: LinkObject[],
    neighbors: NodeObject[],
    url: string,
  }
}

const highlightNodes = new Set();
const highlightLinks = new Set();
let hightlightedNode : NodeObject | null = null;

const showHighlightedNodeInfo = (node: NodeObject | null) => {
  const $info = document.getElementById('hightlighted-node')
  if (!$info) return;
  if (node === null) {
    $info.style.display = 'none';
    return;
  }
  $info.style.display = 'flex';
  const url = $info.querySelector('h4')!;
  const neighbors = $info.querySelector('p')!;
  url.textContent = node.url;
  neighbors.textContent = `Neighbors: ${node.links.length}`;
}

// Each origin will have a button to stop traversing it
const displayActiveOrigins = () => {
  $activeOrigins.innerHTML = '';
  $activeOrigins.style.pointerEvents = 'all';
  activeOrigins.forEach((origin) => {
    const $origin = document.createElement('div');
    $origin.classList.add('origin');
    const $originName = document.createElement('h4');
    $originName.textContent = origin;
    const $originStop = document.createElement('button');
    $originStop.textContent = 'Detener';
    $originStop.addEventListener('click', () => {
      console.log(JSON.stringify({
        subject: 'stop-origin',
        payload: origin
      }))
      ws.send(JSON.stringify({
        subject: 'stop-origin',
        payload: origin
      }));
    });
    $origin.appendChild($originName);
    $origin.appendChild($originStop);
    $activeOrigins.appendChild($origin);
  })
}

const graph = ForceGraph()($root)
  .onNodeHover((node) => {
    hightlightedNode = node ?? null;
    showHighlightedNodeInfo(node);

    highlightNodes.clear();
    highlightLinks.clear();
    if (node) {
      highlightNodes.add(node);
      node.neighbors.forEach(neighbor => highlightNodes.add(neighbor));
      node.links.forEach(link => highlightLinks.add(link));
    }
  })
  .onLinkHover(link => {
    highlightNodes.clear();
    highlightLinks.clear();

    if (link) {
      highlightLinks.add(link);
      highlightNodes.add(link.source);
      highlightNodes.add(link.target);
    }
  })
  .backgroundColor('#0b0b0b')
  .linkColor(() => '#9e90a6')
  .linkWidth(link => highlightLinks.has(link) ? 3 : 0.5)
  .nodeRelSize(1.5)
  .nodeCanvasObjectMode(node => highlightNodes.has(node) ? 'before' : undefined)
  .nodeCanvasObject((node, ctx) => {
    // add ring just for highlighted nodes
    ctx.beginPath();
    // consider the current node size
    ctx.arc(
      node.x!,
      node.y!,
      (node.neighbors.length * 0.2) + 2,
      0,
      2 * Math.PI,
      false
    );
    ctx.fillStyle = node === hightlightedNode ? 'red' : 'orange';
    ctx.fill();
  })
  .nodeAutoColorBy((node) => {
    const domain = node.id as string;
    const lastDot = domain.lastIndexOf('.');

    console.log(domain.substring(0, lastDot))
    return domain.substring(0, lastDot);
  })
  .graphData(gData);

graph.nodeVal((node) => {
  const { links, nodes } = graph.graphData();
  const gatheredLinks = links.filter((l) => {
    if (!l.source || !l.target) return false;
    const source = l.source as NodeObject;
    const target = l.target as NodeObject;
    return source.id === node.id || target.id === node.id
  });
  const gatheredNeighbors = gatheredLinks.map((l) => {
    if (!l.source || !l.target) return null;
    const source = l.source as NodeObject;
    const target = l.target as NodeObject;
    return source.id === node.id ? target : source;
  }).filter((n) => n !== null) as NodeObject[];
  node.links = gatheredLinks;
  node.neighbors = gatheredNeighbors;
  return (gatheredLinks.length + 1) * 0.8;
})

// make that, the bigger the node, the less the force it exerts on its neighbors.
// graph.d3Force('charge').strength((node) => {
//   return -2000 / node.sizeProperty;
// })
// graph.d3Force('charge').st

// listen on input send
$input.addEventListener('keyup', (e) => {
  if (e.key === 'Enter') {
    if (!isValidURL($input.value)) {
      console.log(`invalid: ${$input.value}`)
      return;
    }

    ws.send(JSON.stringify({
      subject: 'new-url',
      payload: $input.value
    }));
    activeOrigins.push($input.value);
    displayActiveOrigins();
  }
});

type IterlinkerMessage = {
  type: "UrlMessage"
  is_invalid: boolean
} | {
  type: "FinishMessage",
  origin: string
} | {
  type: "ResultMessage"
  for_url: string,
  for_domain: string,
  linked_url: string,
  linked_domain: string,
};

const linksStack: string[] = [];

ws.addEventListener('message', (m) => {
  if (typeof m.data !== 'string') {
    return;
  }
  let msg = {} as IterlinkerMessage;
  try {
    msg = JSON.parse(m.data) as IterlinkerMessage;
  }
  catch (e) {
    console.error('Failed to parse:', m.data);
    return;
  }
  switch (msg.type) {
    case 'UrlMessage': {
      const { is_invalid } = msg;
      if (is_invalid) {
        console.log(`invalid input: ${$input.value}`)
      }
      break;
    }
    case 'FinishMessage': {
      console.log('finish');
      const { origin } = msg;
      activeOrigins = activeOrigins.filter((o) => o !== origin);
      displayActiveOrigins();
      break;
    }
    case 'ResultMessage': {
      const { nodes, links } = graph.graphData();
      const { for_domain, for_url, linked_domain, linked_url } = msg;
      const for_url_exists = nodes.find(n => n.id === for_domain);
      const linked_url_exists = nodes.find(n => n.id === linked_domain);
      // console.log({ for_domain, linked_domain })
      linksStack.push(`${for_url} 👉 ${linked_url}`);

      let new_nodes = [...nodes];
      if (!for_url_exists) {
        new_nodes = [...new_nodes, {
          id: for_domain,
          links: [],
          url: for_url
      }];
      }
      if (!linked_url_exists) {
        new_nodes = [...new_nodes, {
          id: linked_domain,
          links: [],
          url: linked_url
        }];
      }

      graph.graphData({
        nodes: new_nodes,
        links: [...links, { source: for_domain, target: linked_domain }]
      });
      break;
    }
  }
});

window.setInterval(() => {
  if (activeOrigins.length === 0) {
    $currentLink.textContent = '';
    linksStack.length = 0;
  }
  $currentLink.textContent = linksStack.pop() ?? '';
}, 50);

const updateSize = () => {
  graph.width(window.innerWidth)
  graph.height(window.innerHeight + 4)
}

window.addEventListener('resize', updateSize);
updateSize();
