import { useState, useEffect, useRef } from 'react'
import Controls from './Controls';

function Universe({ Life, Renderer, memory }) {
  const canvasRef = useRef(null);
  const isStepping = useRef(false);
  const animationFrame = useRef(null);
  const queueDrawCell = useRef(null);
  const queueSetCells = useRef([]);
  const lastMousePos = useRef(null);

  const [playing, _setPlaying] = useState(false);
  const playingRef = useRef(playing);
  const setPlaying = (data) => {
    playingRef.current = data;
    _setPlaying(data);
  }

  const [fpsInfo, setFpsInfo] = useState('');

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
    Renderer.set_size(canvas.width, canvas.height, 1);
    // Renderer.set_size(canvas.width, canvas.height, window.devicePixelRatio);
    if (!playing) draw();
  }

  function offset(direction) {
    switch (direction) {
      case 'up':
        Renderer.move_offset(0, 100);
        break;
      case 'down':
        Renderer.move_offset(0, -100);
        break;
      case 'left':
        Renderer.move_offset(100, 0);
        break;
      case 'right':
        Renderer.move_offset(-100, 0);
        break;
    }
    if (!playing) draw();
  }

  function zoomOut(out) {
    Renderer.zoom_centered(out);
    if (!playing) draw();
  }

  function step() {
    Life.advance(1);
    if (!playing) draw();
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

    if (0.1 * new_cell_width < 1) {
      center_x = Math.round(center_x * new_cell_width);
      center_y = Math.round(center_y * new_cell_width);
    }
    else {
      center_x = Math.round((center_x * new_cell_width * 1.1));
      center_y = Math.round((center_y * new_cell_width * 1.1));
    }

    Renderer.zoom_to(new_cell_width);
    Renderer.center_view(center_x, center_y);
    if (!playing) draw();
  }

  function getMousePos(canvas, event) {
    const rect = canvas.getBoundingClientRect();
    return {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
  }

  const drag = (e) => {
    if (lastMousePos !== null) {
      let dx = Math.round(e.clientX - lastMousePos[0]);
      let dy = Math.round(e.clientY - lastMousePos[1]);
      Renderer.move_offset(dx, dy);
      lastMousePos[0] += dx;
      lastMousePos[1] += dy;
    }
  }

  const drawCells = (e) => {
    const mouse_pos = getMousePos(canvasRef.current, e);

    queueDrawCell.current = [mouse_pos.x, mouse_pos.y];

    queueSetCells.current.push([mouse_pos.x, mouse_pos.y, true]);
  }
  
  function playLoop() {
    if (isStepping.current) {
      step();
    }
    draw();
    animationFrame.current = requestAnimationFrame(playLoop);
  }

  function play(play) {
    if (play) {
      animationFrame.current = requestAnimationFrame(playLoop);
    }
    else {
      cancelAnimationFrame(animationFrame.current);
      animationFrame.current = null;
    }
  }

  function playWithStep(step) {
    isStepping.current = step;
    play(step);
    setPlaying(step);
  }

  function draw() {
    if (queueDrawCell.current) {
      Renderer.draw_cell(queueDrawCell.current[0], queueDrawCell.current[1])
    }

    if (queueSetCells.current.length > 0) {
      for (let x = 0; x < queueSetCells.current.length; x++) {
        Life.set_cell(queueSetCells.current[x][0], queueSetCells.current[x][1], queueSetCells.current[x][2]);
      }
      queueSetCells.current = [];
    }

    const canvas = canvasRef.current;
    const context = canvas.getContext('2d');

    const width = canvas.width;
    const height = canvas.height;
    const imagePtr = Renderer.get_image_data();
    const imageDataArray = new Uint8ClampedArray(memory.buffer, imagePtr, width * height * 4);
    const imageData = new ImageData(imageDataArray, width, height);
    context.putImageData(imageData, 0, 0);
  }
  
  useEffect(() => {
    // glider
    // const items = [0, 0, 1, 1, 2, 1, 2, 0, 2, -1];

    // acorn
    // const acorn = [0, 0, 1, 0, 1, -2, 3, -1, 4, 0, 5, 0, 6, 0];

    // pulsar
    const pulsar = [1, 2, 1, 3, 1, 4, 2, 1, 3, 1, 4, 1, 1, -2, 1, -3, 1, -4, 2, -1, 3, -1, 4, -1, 
                    -1, -2, -1, -3, -1, -4, -2, -1, -3, -1, -4, -1, -1, 2, -1, 3, -1, 4, -2, 1, -3, 1, -4, 1, 
                    2, 6, 3, 6, 4, 6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4, -6, -2, 6, -3, 6, -4, 6,
                    6, 2, 6, 3, 6, 4, 6, -2, 6, -3, 6, -4, -6, 2, -6, 3, -6, 4, -6, -2, -6, -3, -6, -4];

    // gosper gun
    const items = [24, 0, 22, 1, 24, 1, 12, 2, 13, 2, 20, 2, 21, 2, 34, 2, 35, 2, 11, 3, 15, 3, 20, 3, 21, 
      3, 34, 3, 35, 3, 0, 4, 1, 4, 10, 4, 16, 4, 20, 4, 21, 4, 0, 5, 1, 5, 10, 5, 14, 5, 16, 5, 17, 5, 22, 
      5, 24, 5, 10, 6, 16, 6, 24, 6, 11, 7, 15, 7, 12, 8, 13, 8];

    for (let x = 0; x < pulsar.length; x += 2) {
      items.push(pulsar[x] + 50, pulsar[x + 1]);
      items.push(pulsar[x] - 20, pulsar[x + 1]);
    }

    Life.construct(items);
    console.log(Life.level());
    console.log(Life.expand());
    console.log(Life.convert_rle(Life.expand(), 'gosper gun'));

    resizeCanvas();
    centerView();
    draw();

    const canvas = canvasRef.current;

    canvas.onmousedown = (e) => {
      e.preventDefault();

      if (e.which === 3 || e.which === 2) {
        canvas.addEventListener('contextmenu', (e) => e.preventDefault());

        if (Renderer.get_cell_width() >= 1) {
          drawCells(e);
          window.addEventListener('mousemove', drawCells, true);
        }
      }
      if (e.which === 1) {
        lastMousePos[0] = e.clientX;
        lastMousePos[1] = e.clientY;
        window.addEventListener('mousemove', drag, true);
      }

      if (!playingRef.current) {
        isStepping.current = false;
        play(true);
      }
      return false;
    };

    window.onmouseup = () => {
      lastMousePos.current = null;
      queueDrawCell.current = null;
      window.removeEventListener('mousemove', drawCells, true);
      window.removeEventListener('mousemove', drag, true);
      canvas.removeEventListener('contextmenu', (e) => e.preventDefault());
      if (!playingRef.current) {
        play(false);
        draw()
      };
    };

    canvas.addEventListener('contextmenu', (e) => e.preventDefault());
    
    canvas.onmousewheel = (e) => {
      e.preventDefault();
      const mouse_pos = getMousePos(canvas, e);
      Renderer.zoom_at((e.wheelDelta || -e.detail) < 0, mouse_pos.x, mouse_pos.y);
      draw();
      return false;
    };

    window.addEventListener('resize', () => resizeCanvas());

    return () => {
      play(false);
      window.removeEventListener('resize', () => resizeCanvas());
      window.removeEventListener('mousemove', draw, true);
      window.removeEventListener('mousemove', drag, true);
      canvas.removeEventListener('contextmenu', (e) => e.preventDefault());
    }
  }, []);

  return (
    <div className="Universe">
      <Controls 
        offset={offset}
        zoomOut={zoomOut}
        centerView={centerView}
        step={step}
        play={playWithStep}
        playing={playing}
      />
      <canvas className='Canvas' ref={canvasRef} />
      {/* <p>fpsInfo</p> */}
    </div>
  )
}

export default Universe
