from mitmproxy import http
import time
import re

wait = time.time()
sure = False
safe = False
into = 3921
init = [692]

async def response(flow: http.HTTPFlow) -> None:
  resp = flow.response

  global wait
  global sure

  if flow.metadata.get("check"):
    head = resp.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      till = time.time() - wait
      size = int(size)

      print(f"[ resp | {size} | {till} ]")

      if size in init:
        sure = False
        flow.intercept()
        return
      else:
        flow.intercept()
        return
    else:
      return
  else:
    return

async def request(flow: http.HTTPFlow) -> None:
  reqs = flow.request
  kind = reqs.method
  host = reqs.pretty_host

  global wait
  global sure
  global into
  global safe
  global wait

  if "POST" == kind:
    head = reqs.headers
    size = head.get("Content-Length")
    type = head.get("Content-Type")

    if type and "application/x-protobuf" == type and size and size.isdigit():
      till = time.time() - wait
      size = int(size)

      print(f"[ reqs | {size} | {till} | {head.get("Cookie")} ]")

      if into >= size:
        sure = True
        safe = True
        return
      else:
        if safe:
          wait = time.time()
          safe = False
          return
        else:
          if (into + 256) >= size:
            if till >= 60:
              if re.match(r"^ap\.", host):
                vg_5 = head.get("X-VG-5")
                vg_4 = head.get("X-VG-4")

                if vg_4 or vg_5:
                  return
                else:
                  flow.metadata["check"] = True
              else:
                if sure:
                  return
                else:
                  flow.metadata["check"] = True
                return
            else:
              return
          else:
            return
    else:
      return
  else:
    return
