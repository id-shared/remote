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
          return
    else:
      return
  else:
    return

# if i_int == 4154:
#   data = bytes.fromhex("080812f00352470100b5a55d8f241a1a0e23b59b74255fb5fefe0b8cfedb5b681f9ed7632eda2ad65dd3693996fed5b5e82bf6725951a93ba2638e365cbf121a31e31432bfec50d9286c3e69f19c79d864f6b7c7488cc595c1ef6a614bb26681e8b218ec5a7c79593f713b42be6356a63922d9c73dbf3e1323229b281b442086f92e51aedddf4700b99cdb55af95462a4e0696124d53bfdc8aff0c2c72aa2342ec695af7001fa65c0602ca8bae0e031f7c2b889da8a46b4b89eaca131ad7fdb5e199c1bda46700330e97a336af9207ceb631d736a507de343e5fbcb4481235547614185ca9c239f517c53eaf33e51678fade22685f550f4a12b0f67f633217961b4aa33ddfa6029989e8c583cbf009dac0564b3ae796684f09459eec5b98be37515b973e52d6c34c64c085408c37ad89ff3b9e03d3bbd983cf61c2212f8af4cbc88494c4084e341da85b97cf1dcb1ebbe03e3846a79326b905d983993e43efec44a9da9df2da6d6d2723f3219cd7f5cfc5ca7f8b6a227ccca1af0793922178078f80379554677163e9a398042ac8e52d760e23ddf4d26d4f58c1859ef986da84cd72d06198035c023eb0a57a18e64da8261ef6fb1ebda873b52e680395f2f9842d0091039d68ae649ae8497cad62a0a160b2e08313192b6cb26c2b70af8de5cee16b40561f76448f72c213cd0c5d22000d37bc78ef");
#   o_reqs.headers["Content-Length"] = len(data)
#   o_reqs.content = data
#   return
# else:
#   data = bytes.fromhex("080812ee02524701001e3d8aedf1f7d398c50e554b5db0364360c628a0acfafb7d11e123c1dc3116411793a61fb2b7645a17954aa6116409538fdda0bebbb30a40dd204126eba1c2aab637590cae513401e0e83f48da020c9b150ea553743f5979b8e3ab8dfdbf1e6103373c1e7dcd0e3739cec067171a635f6840662a646aba230501c9430a2a314f3ef7da8b583554f1cc8404a968bb5cf7193e75462bf9384851135fe6c6e1b64cfe427c7be0bdc1812311f44c11e7bcaf608dc03def5a924990e789b0fe3d863231dc09b4c5c4a95d77ffb23d498ba6f917365447f80aed7047e1d1fd412e572d3952bed8fcbde9851f855a13ebad91abf1300548f8e2d6f07e9e01a610cfb1be47db1fe9d32a2f5caf9960f6940f9dedb47ca72988c5bddebf61d799a5572465c85835faee3113bb2cc2c8bff4aef22a9023cb064e73da619b9cd383e5638935646fb3893c2fc11f9d8190b165474309c810963d709beb2180edf4b1957449507c438ec6fe3c8161ec70");
#   o_reqs.headers["Content-Length"] = len(data)
#   o_reqs.content = data
#   return

# if "ap2.vg.ac.pvp.net" == host and prev > 1:
#   flow.intercept()
#   await asyncio.sleep(wait)
#   flow.resume()
#   prev = 0
# else:
#   flow.intercept()
#   prev = prev + 1

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
