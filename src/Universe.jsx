import { useState, useEffect, useRef } from 'react'
import Controls from './Controls';

function Universe({ Life, Renderer, memory }) {
  const canvasRef = useRef(null);
  const animationFrame = useRef(null);
  const [playing, setPlaying] = useState(false);

  function loadRle(pattern) {
    const items = Life.parse_rle(pattern);
    Life.construct(items);
  }

  function resizeCanvas() {
    const canvas = canvasRef.current;
    const width = document.documentElement.clientWidth;
    const height = document.documentElement.clientHeight;
    canvas.width = width;
    canvas.height = height;
    Renderer.set_size(canvas.width, canvas.height, window.devicePixelRatio);
    draw();
  }

  function offset(direction) {
    switch (direction) {
      case 'up':
        Renderer.move_offset(0, -100);
        break;
      case 'down':
        Renderer.move_offset(0, 100);
        break;
      case 'left':
        Renderer.move_offset(-100, 0);
        break;
      case 'right':
        Renderer.move_offset(100, 0);
        break;
    }
    draw();
  }

  function zoomOut(out) {
    Renderer.zoom_centered(out);
    draw();
  }

  function step() {
    Life.advance(1);
    draw();
  }

  function playLoop() {
    step();
    draw();
    animationFrame.current = requestAnimationFrame(playLoop);
  }

  function play(play) {
    if (play) {
      animationFrame.current = requestAnimationFrame(playLoop);
    }
    else {
      cancelAnimationFrame(animationFrame.current);
    }
    setPlaying(play);
  }

  function centerView() {
    const bounds = Life.get_bounds();
    const width = Math.ceil((bounds[0] - bounds[1]) * 1.1);
    const height = Math.ceil((bounds[2] - bounds[3]) * 1.1);

    const width_factor = Math.abs(document.documentElement.clientWidth / width);
    const height_factor = Math.abs(document.documentElement.clientHeight / height);
    const factor = Math.min(width_factor, height_factor);

    var new_cell_width = 1;
    if (factor > 1) {
      while (new_cell_width < factor) {
        new_cell_width *= 2;
      }
      new_cell_width /= 2;
    }
    else {
      while (new_cell_width > factor) {
        new_cell_width /= 2;
      }
    }

    var center_x = (bounds[0] + bounds[1]) / 2;
    var center_y = (bounds[2] + bounds[3]) / 2;

    console.log(`center_x: ${center_x} center_y: ${center_y}`);

    if (0.1 * new_cell_width < 1) {
      center_x = Math.round(center_x * new_cell_width);
      center_y = Math.round(center_y * new_cell_width);
    }
    else {
      center_x = Math.round((center_x * new_cell_width * 1.1));
      center_y = Math.round((center_y * new_cell_width * 1.1));
    }

    console.log(`center_x: ${center_x} center_y: ${center_y}`);
    console.log(`new_cell_width: ${new_cell_width}`);

    Renderer.zoom_to(new_cell_width);
    Renderer.center_view(center_x, center_y);
    draw();
  }
  
  function draw() {
    const canvas = canvasRef.current;
    const context = canvas.getContext('2d');

    const width = canvas.width;
    const height = canvas.height;
    const imagePtr = Renderer.get_image_data();
    console.log('pointer:' + imagePtr)
    console.log(memory)
    const imageDataArray = new Uint8ClampedArray(memory.buffer, imagePtr, width * height * 4);
    const imageData = new ImageData(imageDataArray, width, height);
    context.putImageData(imageData, 0, 0);

    console.log(Life.population());
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

    Life.construct(items);
    console.log(Life.level());
    console.log(Life.expand());
    console.log(Life.convert_rle(Life.expand(), 'gosper gun'));

    Life.advance();
    Life.advance();
    Life.advance();
    Life.advance();
    Life.advance();
    Life.advance();
    Life.advance();
  
    window.addEventListener('resize', () => resizeCanvas());

    resizeCanvas();

    console.log(memory.buffer.byteLength);

    centerView();
    
    draw();

    return () => {

    }
  }, []);

  return (
    <div className="Universe">
      <div id='fps'></div>
      <Controls 
        offset={offset}
        zoomOut={zoomOut}
        centerView={centerView}
        step={step}
        play={play}
        playing={playing}
      />
      <canvas className='Canvas' ref={canvasRef} />
    </div>
  )
}

export default Universe
