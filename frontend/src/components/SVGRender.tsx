import { useEffect, useState } from 'react';
import type { ASTNode } from '@types';

function SVGRender({ ast }: { ast: ASTNode }) {
  const [svgStr, setSvgStr] = useState('');
  const [svgParams, setSvgParams] = useState<[number, number, number]>([20, 30, 10]);

  const renderSVG = async () => {
    const { generate_svg } = await import('logic-parsers');
    setSvgStr(
      generate_svg(ast, ...svgParams)
    );
  };

  useEffect(() => {
    renderSVG();
  });

  return (
    <section>
      <h5 id="params-title">Parámetros</h5>
      <header id="svg-params">
        <div className="svg-param">
          <span>x&#8209;sep</span>
          <input type="range" min={0} max={70} value={svgParams[0]} onInput={(e) => setSvgParams([parseInt(e.currentTarget.value), svgParams[1], svgParams[2]])} />
        </div>
        <div className="svg-param">
          <span>y&#8209;sep</span>
          <input type="range" min={0} max={80} value={svgParams[1]} onInput={(e) => setSvgParams([svgParams[0], parseInt(e.currentTarget.value), svgParams[2]])} />
        </div>
        <div className="svg-param">
          <span>radius</span>
          <input type="range" min={0} max={30} value={svgParams[2]} onInput={(e) => setSvgParams([svgParams[0], svgParams[1], parseInt(e.currentTarget.value)])} />
        </div>
      </header>
      <img
        id="svg-render-img"
        src={`data:image/svg+xml;utf8,${encodeURI(svgStr)}`}
      />
    </section>
  );
}

export default SVGRender;
