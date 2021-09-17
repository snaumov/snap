import React, { useState } from 'react';
import './App.css';
import {invoke} from '@tauri-apps/api/tauri';

// const invoke = window.__TAURI__.invoke;

function App() {
  const [url, setUrl] = useState(null);
  return (
    <div className="App">
      <header className="App-header">
        {url && <span>{url}</span>}
        <button onClick={() => invoke('handle_screenshot_capture').then((url) => setUrl(url))}>Screenshot</button>
      </header>
    </div>
  );
}

export default App;
