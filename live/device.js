import {mouse, Point, Button} from "@nut-tree-fork/nut-js";
import {createServer} from "node:http";
import {readFile} from "node:fs";
import {WebSocketServer} from "ws";

const server = createServer((req, res) => {
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

const wss = new WebSocketServer({ server });

wss.on("connection", (ws) => {
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

server.listen(9998, () => {
  console.log("Server listening on http://localhost:9998");
});
