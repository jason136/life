import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'

import { Life, Renderer } from 'life';
import { memory } from 'life/life_bg';

function App() {
  const [count, setCount] = useState(0);

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

  let node = Life.construct(items);
  console.log(node.level);
  console.log(Life.expand(node));
  console.log(Life.convert_rle(Life.expand(node), 'gosper gun'));

  function loadRle(pattern: string) {
    const items = Life.parse_rle(pattern);
    return Life.construct(items);
  }

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
