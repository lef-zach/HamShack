import { useEffect, useRef, useState } from 'react';
import useSSE from '../hooks/useSSE';

const Waterfall = ({ width = 800, height = 200 }) => {
  const canvasRef = useRef(null);
  const [spectrumData, setSpectrumData] = useState([]);
  const [sdrStatus, setSdrStatus] = useState({});
  
  // Listen for SSE updates
  const sseData = useSSE('http://localhost:3000/api/sse');

  useEffect(() => {
    if (sseData) {
      try {
        const data = JSON.parse(sseData);
        if (data.type === 'sdr_status') {
          setSdrStatus(data.data);
        }
      } catch (error) {
        console.error('Error parsing SSE data:', error);
      }
    }
  }, [sseData]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    
    const drawWaterfall = () => {
      // Clear canvas
      ctx.fillStyle = '#000';
      ctx.fillRect(0, 0, width, height);
      
      if (spectrumData.length === 0) {
        // Draw placeholder
        ctx.fillStyle = '#333';
        ctx.font = '16px monospace';
        ctx.textAlign = 'center';
        ctx.fillText('Waiting for SDR data...', width / 2, height / 2);
        return;
      }

      // Draw spectrum
      const barWidth = width / spectrumData.length;
      
      spectrumData.forEach((value, index) => {
        const x = index * barWidth;
        const normalizedValue = Math.max(0, Math.min(1, (value + 100) / 100)); // Normalize dB values
        const barHeight = normalizedValue * height;
        
        // Color gradient from blue (low) to red (high)
        const hue = 240 - (normalizedValue * 240);
        ctx.fillStyle = `hsl(${hue}, 100%, 50%)`;
        
        ctx.fillRect(x, height - barHeight, barWidth - 1, barHeight);
      });

      // Draw frequency scale
      ctx.fillStyle = '#fff';
      ctx.font = '12px monospace';
      ctx.textAlign = 'left';
      
      if (sdrStatus.frequency) {
        const centerFreq = sdrStatus.frequency / 1e6;
        ctx.fillText(`${centerFreq.toFixed(3)} MHz`, 10, 20);
      }
      
      if (sdrStatus.running !== undefined) {
        const status = sdrStatus.running ? 'RUNNING' : 'STOPPED';
        const statusColor = sdrStatus.running ? '#0f0' : '#f00';
        ctx.fillStyle = statusColor;
        ctx.fillText(`SDR: ${status}`, 10, 40);
      }
    };

    drawWaterfall();
  }, [spectrumData, sdrStatus, width, height]);

  // Simulate spectrum data updates (will be replaced with real SDR data)
  useEffect(() => {
    const interval = setInterval(() => {
      if (sdrStatus.running) {
        // Generate simulated spectrum data
        const newData = Array.from({ length: 256 }, (_, i) => {
          const baseNoise = Math.random() * 20 - 40;
          const signal1 = Math.sin(i / 10) * 10;
          const signal2 = i > 100 && i < 150 ? Math.sin(i / 5) * 15 : 0;
          return baseNoise + signal1 + signal2;
        });
        setSpectrumData(newData);
      }
    }, 100);

    return () => clearInterval(interval);
  }, [sdrStatus.running]);

  const startSDR = async () => {
    try {
      await fetch('http://localhost:3000/api/sdr/start');
    } catch (error) {
      console.error('Failed to start SDR:', error);
    }
  };

  const stopSDR = async () => {
    try {
      await fetch('http://localhost:3000/api/sdr/stop');
    } catch (error) {
      console.error('Failed to stop SDR:', error);
    }
  };

  const setFrequency = async (freq) => {
    try {
      await fetch(`http://localhost:3000/api/sdr/frequency/${freq}`);
    } catch (error) {
      console.error('Failed to set frequency:', error);
    }
  };

  return (
    <div className="waterfall-container">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
        <h3>SDR Waterfall Display</h3>
        <div>
          <button 
            onClick={startSDR} 
            style={{ 
              marginRight: '10px', 
              backgroundColor: sdrStatus.running ? '#0a0' : '#555',
              color: 'white',
              border: 'none',
              padding: '5px 10px',
              borderRadius: '3px',
              cursor: 'pointer'
            }}
          >
            Start SDR
          </button>
          <button 
            onClick={stopSDR}
            style={{ 
              marginRight: '10px',
              backgroundColor: !sdrStatus.running ? '#a00' : '#555',
              color: 'white',
              border: 'none',
              padding: '5px 10px',
              borderRadius: '3px',
              cursor: 'pointer'
            }}
          >
            Stop SDR
          </button>
          <button 
            onClick={() => setFrequency(14200000)}
            style={{ 
              marginRight: '10px',
              backgroundColor: '#555',
              color: 'white',
              border: 'none',
              padding: '5px 10px',
              borderRadius: '3px',
              cursor: 'pointer'
            }}
          >
            14.2 MHz
          </button>
          <button 
            onClick={() => setFrequency(7100000)}
            style={{ 
              backgroundColor: '#555',
              color: 'white',
              border: 'none',
              padding: '5px 10px',
              borderRadius: '3px',
              cursor: 'pointer'
            }}
          >
            7.1 MHz
          </button>
        </div>
      </div>
      
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        style={{ border: '1px solid #ccc', background: '#000' }}
      />
      
      <div style={{ fontSize: '12px', color: '#666', marginTop: '10px' }}>
        <div>Frequency: {sdrStatus.frequency ? (sdrStatus.frequency / 1e6).toFixed(3) + ' MHz' : 'N/A'}</div>
        <div>Sample Rate: {sdrStatus.sample_rate ? (sdrStatus.sample_rate / 1e6).toFixed(1) + ' MS/s' : 'N/A'}</div>
        <div>Gain: {sdrStatus.gain || 'N/A'} dB</div>
      </div>
    </div>
  );
};

export default Waterfall;