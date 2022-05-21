import { useCallback, useState } from 'react'
import logo from './logo.svg'
import { invoke } from '@tauri-apps/api/tauri'
import { message } from '@tauri-apps/api/dialog';
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  const onClickWithName = useCallback(() => {
    invoke('with_message', { message: { name: 'Tauri' }})
      .then(msg => {
        if (typeof msg === 'object' && msg !== null && 'message' in msg) {
          message((msg as { message: string }).message);
        }
      });
  }, []);

  const onClickMessage = useCallback(() => {
    message('Hello');
  }, []);

  const onClickIncrement = useCallback(() => {
    invoke('countup').then(value => {
      if (typeof value === 'number') {
        setCount(value);
      }
    });
  }, []);

  return (
    <div className="app">
      <button onClick={onClickMessage}>show message</button>
      <button onClick={onClickWithName}>show message</button>
      <button onClick={onClickIncrement}>count: {count}</button>
    </div>
  )
}

export default App
