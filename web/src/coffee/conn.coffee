#!/usr/bin/env coffee

export default (wasm)=>
  {connect, ws, default:init} = wasm
  await init()

  retry = 0

  w = ws(
    "ws://127.0.0.1:4910"
    =>
      retry = 0
      return
    =>
      if retry < 99
        ++retry
      setTimeout(
        =>
          connect(w)
          return
        retry*99
      )
      return
  )
  w
