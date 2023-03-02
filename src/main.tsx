import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

import init, { init_panic_hook } from 'life';

init().then(() => {
  init_panic_hook();
  console.log('WASM loaded');
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
