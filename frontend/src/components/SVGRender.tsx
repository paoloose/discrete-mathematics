import { useEffect, useState } from 'preact/hooks';

function SVGRender({ ast }: { ast: any }) {
  const [svgStr, setSvgStr] = useState('');

  const renderSVG = async () => {
    const { generate_svg } = await import('logic-parsers');
    setSvgStr(
      generate_svg(ast, 20, 30, 10)
    );
  }

  useEffect(() => {
    renderSVG();
  });

  return (
    <img
      id="svg-render"
      src={`data:image/svg+xml;utf8,${encodeURI(svgStr)}`}
    />
  );
}

export default SVGRender;
