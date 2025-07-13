import {statSync, readFileSync} from "node:fs";
import {createServer} from "node:http";

const PORT = 9999;
const JPEG_PATH = '0.jpg';

function getLastFrameInfo() {
  try {
    const stats = statSync(JPEG_PATH);
    const buf = readFileSync(JPEG_PATH);
    return { mtimeMs: stats.mtimeMs, data: buf };
  } catch (e) {
    return null;
  }
}

createServer((req, res) => {
  if (req.url.startsWith('/stream')) {
    res.writeHead(200, {
      'Content-Type': 'multipart/x-mixed-replace; boundary=frame',
      'Cache-Control': 'no-cache',
      'Connection': 'close',
      'Pragma': 'no-cache'
    });

    let closed = false;
    req.on('close', () => { closed = true; });

    let lastSentMTime = 0;
    let lastSentData = '';

    function streamLoop() {
      if (closed) return;

      const frameInfo = getLastFrameInfo();
      if (frameInfo && frameInfo.mtimeMs !== lastSentMTime) {
        const data = frameInfo.data;
        const hash = data.toString('base64');

        if (hash !== lastSentData && data.length > 0) {
          lastSentMTime = frameInfo.mtimeMs;
          lastSentData = hash;
          res.write(
            `--frame\r\nContent-Type: image/jpeg\r\nContent-Length: ${data.length}\r\n\r\n`
          );
          res.write(data);
          res.write('\r\n');
        }
      }
      setTimeout(streamLoop, 64);
    }
    streamLoop();

  } else {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(
      `<!DOCTYPE html>
<html>
<head>
  <title>Custom RDP</title>
  <style>
    body, html {
      margin: 0;
      padding: 0;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      background-color: #EFEFEFFF;
    }
    img {
      max-width: 100%;
      max-height: 100%;
      height: auto;
      width: auto;
    }
  </style>
</head>
<body>
  <img id="mjpeg" src="/stream" alt="MJPEG Stream">
  <script>
    const img = document.getElementById('mjpeg');
    let watchdog = null;

    function restartStream() {
      img.src = '/stream?' + Date.now();
    }

    function resetWatchdog() {
      clearTimeout(watchdog);
      watchdog = setTimeout(() => {
        console.warn('MJPEG stream stalled, retrying...');
        restartStream();
      }, 5000); // 2 seconds, adjust as needed
    }

    img.onload = () => {
      // Every new JPEG, reset the watchdog
      resetWatchdog();
    };

    img.onerror = () => {
      // This still handles hard errors
      console.error('Stream error, retrying...');
      restartStream();
    };

    // Start watchdog at page load
    resetWatchdog();
  </script>
</body>
</html>`
    );
  }
}).listen(PORT, () => {
  console.log(`http://localhost:${PORT}/stream`);
});
