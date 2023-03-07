import { useState, useEffect } from 'react'
import './App.css'

import init, { Life, Renderer, init_panic_hook } from 'life';

import Universe from './Universe';

function App() {
  const [wasmInitialized, setWasmInitialized] = useState(false);
  const [memory, setMemory] = useState(null);

  useEffect(() => {
    init().then((InitOutput) => {
      init_panic_hook();
      console.log('WASM loaded');
      setWasmInitialized(true);
      setMemory(InitOutput.memory);
    });
  }, []);

  return (
    <div className="App">
      {wasmInitialized ? 
      <div>
        <Universe Life={Life} Renderer={Renderer} memory={memory}/> 
      </div>
      : 
      <div>Loading...</div>}
    </div>
  )
}

export default App
