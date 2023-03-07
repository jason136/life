import { useState, useEffect } from 'react'

function Controls({ offset, zoomOut, centerView, step, play, playing }) {
  return (
    <div className="Controls">
      <nav>
        <ul>
          <li>
            <button onClick={() => offset('up')}>up</button>
          </li>
          <li>
            <button onClick={() => offset('down')}>down</button>
          </li>
          <li>
            <button onClick={() => offset('left')}>left</button>
          </li>
          <li>
            <button onClick={() => offset('right')}>right</button>
          </li>
          <li>
            <button onClick={() => zoomOut(false)}>in</button>
          </li>
          <li>
            <button onClick={() => zoomOut(true)}>out</button>
          </li>
          <li>
            <button onClick={() => centerView()}>center</button>
          </li>
          <li>
            <button onClick={() => step()}>step</button>
          </li>
          <li>
            {
              playing ? <button onClick={() => play(false)}>pause</button> : <button onClick={() => play(true)}>play</button>
            }
          </li>
        </ul>
      </nav>
    </div>
  )
}

export default Controls
