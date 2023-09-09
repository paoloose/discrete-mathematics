function SVGRender({ svgStr }: { svgStr: string }) {

  return (
    <img src={`data:image/svg+xml;utf8,${encodeURI(svgStr)}`} />
  );
}

export default SVGRender;
