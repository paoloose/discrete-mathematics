function SVGRender({ svgStr }: { svgStr: string }) {

  return (
    <div dangerouslySetInnerHTML={{ __html: svgStr }}></div>
  );
}

export default SVGRender;
