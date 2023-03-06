import { useState, useEffect, useRef } from 'react'
import './App.css'

function Universe({ Life, Renderer, memory }) {
  const canvasRef = useRef(null);
  let canvas, context, node, renderer;

  function loadRle(pattern) {
    const items = Life.parse_rle(pattern);
    return Life.construct(items);
  }

  function resizeCanvas() {
  }
  
  function draw() {
    let renderer = Renderer.new();

    const width = document.documentElement.clientWidth;
    const height = document.documentElement.clientHeight;
    canvas.width = width;
    canvas.height = height;
    renderer.set_size(width, height, window.devicePixelRatio);

    const imagePtr = renderer.get_image_data(node);
    console.log('pointer:' + imagePtr)
    console.log(memory)
    const imageDataArray = new Uint8ClampedArray(memory.buffer, imagePtr, width * height * 4);
    const imageData = new ImageData(imageDataArray, width, height);
    context.putImageData(imageData, 0, 0);
  }
  
  useEffect(() => {
    // glider
    // const items = [0, 0, 1, 1, 2, 1, 2, 0, 2, -1];

    // acorn
    // const items = [0, 0, 1, 0, 1, -2, 3, -1, 4, 0, 5, 0, 6, 0];

    // pulsar
    // const items = [1, 2, 1, 3, 1, 4, 2, 1, 3, 1, 4, 1, 1, -2, 1, -3, 1, -4, 2, -1, 3, -1, 4, -1, 
    //                 -1, -2, -1, -3, -1, -4, -2, -1, -3, -1, -4, -1, -1, 2, -1, 3, -1, 4, -2, 1, -3, 1, -4, 1, 
    //                 2, 6, 3, 6, 4, 6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4, -6, -2, 6, -3, 6, -4, 6,
    //                 6, 2, 6, 3, 6, 4, 6, -2, 6, -3, 6, -4, -6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4];

    // gosper gun
    const items = [24, 0, 22, 1, 24, 1, 12, 2, 13, 2, 20, 2, 21, 2, 34, 2, 35, 2, 11, 3, 15, 3, 20, 3, 21, 
      3, 34, 3, 35, 3, 0, 4, 1, 4, 10, 4, 16, 4, 20, 4, 21, 4, 0, 5, 1, 5, 10, 5, 14, 5, 16, 5, 17, 5, 22, 
      5, 24, 5, 10, 6, 16, 6, 24, 6, 11, 7, 15, 7, 12, 8, 13, 8];

    node = Life.construct(items);
    console.log(node.level());
    console.log(Life.expand(node));
    console.log(Life.convert_rle(Life.expand(node), 'gosper gun'));

    node = Life.advance(node);
    node = Life.advance(node);
    node = Life.advance(node);
    node = Life.advance(node);
    node = Life.advance(node);
    node = Life.advance(node);
    node = Life.advance(node);
  
    renderer = Renderer.new();

    canvas = canvasRef.current;
    context = canvas.getContext('2d');
  
    window.addEventListener('resize', () => resizeCanvas());

    resizeCanvas();

    console.log(memory.buffer.byteLength);
    draw();
  
    renderer.zoom_centered(true);

    return () => {

    }
  }, []);

  return (
    <div className="Universe">
      <div id='fps'></div>
      <canvas ref={canvasRef} />
    </div>
  )
}

export default Universe
