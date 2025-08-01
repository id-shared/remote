from mitmproxy import http
import time

wait = time.time()
upto = 60 * 60

till = 3921
safe = 0
anew = 0

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
  global anew

  print(f"| {kind} | {host} | {port} | {urn} | {url} |")

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      i_int = int(size)

      print(f"| {safe} |")

      if till >= i_int:
        safe = 4
        anew = 1
        return
      else:
        if anew == 0:
          if till >= (i_int - 256):
            if (time.time() - wait) >= 60:
              if safe == 0:
                safe = 4
                return
              else:
                safe = min(0, safe - 1)
                flow.metadata["intercept"] = True
                return
            else:
              return
          else:
            return
        else:
          anew = min(0, anew - 1)
          if anew == 0:
            wait = time.time()
            return
          else:
            return
    else:
      return
  else:
    return
