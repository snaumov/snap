import logo from './logo.svg';
import './App.css';
import {invoke} from '@tauri-apps/api/tauri';

// const invoke = window.__TAURI__.invoke;

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={() => invoke('take_screenshot')}>Screenshot</button>
      </header>
    </div>
  );
}

export default App;
