from mitmproxy import http
import time

wait = time.time()
upto = 60 * 60
safe = False
into = 3921
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
  global into
  global curr
  global safe

  print(f"| {kind} | {host} | {port} | {urn} | {url} |")

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      i_int = int(size)

      print(f"| {curr} | {safe} |")

      if into >= i_int:
        safe = True
        curr = 0
        return
      else:
        if safe:
          wait = time.time()
          safe = False
        else:
          if into >= (i_int - 256):
            if (time.time() - wait) >= 60:
              curr = curr + 1

              if curr == 5:
                curr = 0
                return
              else:
                flow.metadata["intercept"] = True
                return
            else:
              return
          else:
            return
    else:
      return
  else:
    return
