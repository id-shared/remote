from mitmproxy import http
import asyncio

safe = 3921
curr = 0
async def request(flow: http.HTTPFlow) -> None:
  reqs = flow.request
  kind = reqs.method
  path = reqs.path
  host = reqs.pretty_host
  port = reqs.port
  urn = f"https://{host}:{port}{path}"
  url = reqs.url
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
            flow.intercept()
            return
          else:
            return
        else:
          curr = min(0, curr - 1)
          flow.intercept()
          await asyncio.sleep(40)
          flow.resume()
          return
    else:
      return
  else:
    return

# flow.request.headers["Host"] = "ap2.vg.ac.pvp.net"
# flow.request.host = "ap2.vg.ac.pvp.net"
# flow.request.port = 443

# flow.response = http.Response.make(
#   status_code=200,
#   content=prev.content,
#   headers=dict(prev.headers)
# )

# flow.intercept()
# await asyncio.sleep(60)
# flow.resume()
