from mitmproxy import http
import time

wait = time.time()
upto = 60 * 60

till = 3921
safe = 0
curr = 0

async def response(flow: http.HTTPFlow) -> None:
  global upto

  if flow.metadata.get("intercept"):
    if upto >= (time.time() - wait):
      flow.intercept()
    else:
      return
  else:
    return

async def request(flow: http.HTTPFlow) -> None:
  reqs = flow.request
  kind = reqs.method
  path = reqs.path
  host = reqs.pretty_host
  port = reqs.port
  urn = f"https://{host}:{port}{path}"
  url = reqs.url

  global wait
  global till
  global safe
  global curr

  print(f"| {kind} | {host} | {port} | {urn} | {url} |")

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      i_int = int(size)

      print(f"| {safe} |")

      if till >= i_int:
        safe = 0
        curr = 1
        return
      else:
        if curr == 0:
          if till >= (i_int - 256):
            if (time.time() - wait) >= 60:
              safe = safe + 1

              if safe == 5:
                safe = 0
                return
              else:
                flow.metadata["intercept"] = True
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
