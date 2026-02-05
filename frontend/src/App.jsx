import { useState } from 'react';
import Waterfall from './components/Waterfall';
import useSSE from './hooks/useSSE';
import './App.css';

function App() {
  const [count, setCount] = useState(0);
  const sseData = useSSE('http://localhost:3000/api/sse');
  const [backendConnected, setBackendConnected] = useState(false);

  // Check backend connection
  useState(() => {
    const checkConnection = async () => {
      try {
        const response = await fetch('http://localhost:3000/api/health');
        setBackendConnected(response.ok);
      } catch {
        setBackendConnected(false);
      }
    };

    checkConnection();
    const interval = setInterval(checkConnection, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="App">
      <header className="app-header">
        <h1>🚀 HamShack - Next Generation Ham Radio Dashboard</h1>
        <div className="status-bar">
          <span className={`status ${backendConnected ? 'connected' : 'disconnected'}`}>
            Backend: {backendConnected ? '✅ Connected' : '❌ Disconnected'}
          </span>
          <span>SSE Updates: {count}</span>
        </div>
      </header>

      <main className="app-main">
        <section className="dashboard-section">
          <h2>SDR Integration</h2>
          <Waterfall width={800} height={200} />
        </section>

        <section className="dashboard-section">
          <h2>Real-time Data</h2>
          <div className="data-panel">
            <pre>{sseData || 'Waiting for SSE data...'}</pre>
          </div>
        </section>

        <section className="dashboard-section">
          <h2>Quick Actions</h2>
          <div className="action-buttons">
            <button onClick={() => setCount(count + 1)}>
              Test Counter: {count}
            </button>
            <button onClick={() => fetch('http://localhost:3000/api/health')}>
              Health Check
            </button>
          </div>
        </section>
      </main>

      <footer className="app-footer">
        <p>Built with Rust + React • Optimized for Raspberry Pi • SDR + AI Integration</p>
      </footer>
    </div>
  );
}

export default App;
