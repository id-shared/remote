import * as Nut from "@nut-tree-fork/nut-js";
import {createServer} from "node:http";
import {readFile, watchFile} from "node:fs";
import {join} from "node:path";
import {WebSocketServer} from "ws";

const {Button, Point, mouse} = Nut.default;

const server = createServer((req, res) => {
  if (req.url === '/rdp') {
    readFile(join("ws.html"), (err, data) => {
      if (err) {
        res.writeHead(500, { 'Content-Type': 'text/plain' });
        res.end('Internal Server Error');
        return;
      }
      res.writeHead(200, { 'Content-Type': 'text/html' });
      res.end(data);
    });
  } else {
    res.writeHead(404, { 'Content-Type': 'text/plain' });
    res.end('Not Found');
  }
});

// Create WebSocket server
const wss = new WebSocketServer({ server });

// Path to the file to watch
const FILE_PATH = '1.jpg';

// Utility to convert file to Data URL
const send = (file) => (ws) => {
  return readFile(file, (err, data) => {
    if (!err && data.length > 0) {
      // Infer the mime-type (assuming jpg)
      const base64 = data.toString('base64');
      const dataUrl = `data:image/jpeg;base64,${base64}`;

      ws.send(JSON.stringify({ type: 'frame', data: dataUrl }));
    }
  });
};

// Store all connected clients
const clients = new Set();

wss.on('connection', (ws) => {
  clients.add(ws);

  send(FILE_PATH)(ws);

  ws.on('close', () => {
    clients.delete(ws);
  });
});

// Keep track of last sent mtime to avoid duplicate sends
let lastMtime = null;

// Watch the file for changes (using polling for reliability)
watchFile(FILE_PATH, { interval: 1 }, async (curr, prev) => {
  if (curr.mtime > prev.mtime) {
    // File changed
    try {
      // Send to all connected clients
      for (let ws of clients) {
        if (ws.readyState === WebSocket.OPEN) {
          send(FILE_PATH)(ws);
        }
      }
    } catch (err) {
      console.error('Failed to convert or send file:', err);
    }
  }
});

server.listen(9999);

const server2 = createServer((req, res) => {
  if (req.url === "/") {
    readFile("device.html", (err, data) => {
      if (err) {
        res.writeHead(500);
        res.end("Error loading file");
      } else {
        res.writeHead(200, { "Content-Type": "text/html" });
        res.end(data);
      }
    });
  } else {
    res.writeHead(404);
    res.end("Not found");
  }
});

const wss2 = new WebSocketServer({ server: server2 });

wss2.on("connection", (ws) => {
  const does = async (position, name, type) => {
    switch (name in position) {
      case true:
        switch (position[name]) {
          case true:
            return mouse.pressButton(type);
          default:
            return mouse.releaseButton(type);
        }
      default:
        return true;
    }
  };

  ws.on("message", async (message) => {
    const data = JSON.parse(message);

    await does(data, "rmb", Button.RIGHT);
    await does(data, "lmb", Button.LEFT);

    switch ("x" in data && "y" in data) {
      case true:
        return await mouse.move([new Point(data.x, data.y)]);
      default:
        return true;
    }
  });

  ws.on("close", () => {
    console.log("Client disconnected");
  });
});

server2.listen(9998);
