from mitmproxy import http
import time

wait = time.time()
safe = 3921
curr = 0

async def response(flow: http.HTTPFlow) -> None:
  if flow.metadata.get("intercept_response"):
    await flow.intercept()

async def request(flow: http.HTTPFlow) -> None:
  reqs = flow.request
  kind = reqs.method
  path = reqs.path
  host = reqs.pretty_host
  port = reqs.port
  urn = f"https://{host}:{port}{path}"
  url = reqs.url

  global wait
  global safe
  global curr

  print(f"| {kind} | {host} | {port} | {urn} | {url} |")

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      i_int = int(size)

      print(f"| {i_int} |")

      if safe >= i_int:
        curr = 1
        return
      else:
        if curr == 0:
          if safe >= (i_int - 256):
            if (time.time() - wait) > 60:
              flow.metadata["intercept_response"] = True
              return
            else:
              return
          else:
            return
        else:
          curr = min(0, curr - 1)
          if curr == 0:
            wait = time.time()
            return
          else:
            return
    else:
      return
  else:
    return
