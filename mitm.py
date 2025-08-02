from mitmproxy import http
import time

wait = time.time()
upto = 60 * 60
safe = False
into = 3921

async def response(flow: http.HTTPFlow) -> None:
  resp = flow.response

  global upto

  if flow.metadata.get("check"):
    head = resp.headers
    size = head.get("Content-Length")

    i_int = int(size)

    print(f"| {flow.metadata.get("count")} | {i_int} |")

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
  global safe

  print(f"| {kind} | {host} | {port} | {urn} | {url} |")

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      i_int = int(size)

      if into >= i_int:
        safe = True
        return
      else:
        if safe:
          wait = time.time()
          safe = False
        else:
          if (into + 256) >= i_int:
            if (time.time() - wait) >= 60:
              flow.metadata["count"] = i_int
              flow.metadata["check"] = True
            else:
              return
          else:
            return
    else:
      return
  else:
    return
