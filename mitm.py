from mitmproxy import http
import time
import re

wait = time.time()
upto = [692]
safe = False
into = 3921

async def response(flow: http.HTTPFlow) -> None:
  resp = flow.response

  if flow.metadata.get("check"):
    head = resp.headers
    size = head.get("Content-Length")
    size = int(size)

    print(f"[ resp | {size} ]")

    if size in upto:
      flow.intercept()
    else:
      return
  else:
    return

async def request(flow: http.HTTPFlow) -> None:
  reqs = flow.request
  kind = reqs.method
  host = reqs.pretty_host

  global wait
  global into
  global safe

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      size = int(size)

      if into >= size:
        safe = True
        return
      else:
        if safe:
          wait = time.time()
          safe = False
        else:
          if (into + 256) >= size:
            till = time.time() - wait
            if till >= 60:
              print(f"[ reqs | {till} | {size} ]")

              if re.match(r"^ap2", host) or re.match(r"^ap", host):
                flow.metadata["count"] = size
                flow.metadata["check"] = True
                # flow.intercept()
              else:
                flow.intercept()
                return
            else:
              return
          else:
            return
    else:
      return
  else:
    return
