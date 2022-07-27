#!/usr/bin/env coffee

#import fsline from '@rmw/fsline'
import thisdir from '@rmw/thisdir'
import {resolve,join,dirname} from 'path'
import {readFile,writeFile} from 'fs/promises'
import {extract_li} from './extract'
import {upperFirst, camelCase} from 'lodash-es'

PWD = thisdir(import.meta)
ROOT = dirname dirname PWD
RUST = join(ROOT,'rust')

UTF8 = 'utf8'

CLS_MAP = {
  'impl AsRef<str>':'String'
}

read = (fp)=>
  readFile join(RUST,fp), UTF8

write = (fp, txt)=>
  writeFile join(RUST,fp), txt


modify = (fp, gen)=>
  start = '// code_gen <'
  end = '// >'
  cmd = await read fp
  begin_pos = cmd.indexOf(start)+start.length
  if begin_pos < 0
    console.log fp,'no',start
    return
  end_pos = cmd.indexOf(end,begin_pos)
  if end_pos < 0
    console.log fp,'no',end
    return

  loop
    if cmd.charAt(--end_pos).trim() != ''
      break

  write fp, cmd[..begin_pos] + gen(cmd[begin_pos+1..end_pos]) + cmd[end_pos+1..]

enum_name = (i)=>
  i.replace(/[<,>]/g,'')



fn_li = (txt, prefix)=>
  api_cmd = []
  for fn from extract_li txt, prefix+' ',"{"
    pos = fn.indexOf('(')
    if pos > 0
      name = fn[...pos]
      has_return = fn.lastIndexOf('->')
      rt = fn[has_return+2..].trim()
      rt = rt[7...-1]
      if rt == '()'
        rt = undefined

      args = fn[pos+1...fn.lastIndexOf(')',fn.lastIndexOf('->'))].split(",")
      args.shift()
      args = args.map((i)=>i.split(":").map((x)=>x.trim())).filter Boolean

      for i from args
        cls = i[1]
        i[1] = CLS_MAP[cls] or cls

      cmd = upperFirst(camelCase(name))
      api_cmd.push [cmd, args, rt, name]
  api_cmd

export default main = =>
  li = fn_li.bind(null,await read 'net/src/api/cmd.rs')

  api_cmd = li 'pub async fn'
  async = new Set(api_cmd.map (i)=>i[0])
  api_cmd = api_cmd.concat li 'pub fn'

  await Promise.all [
    modify(
      'ws/src/cmd.rs'
      (txt)=>
        space = '    '
        li = []
        for [cmd, args, rt, name] from api_cmd
          args_pass = args.map((i)=>i[0]).join(', ')
          if args.length
            args_tuple = "(#{args_pass})"
          else
            args_tuple = ''
          txt = "Cmd::#{cmd}#{args_tuple} => "
          call = "api.#{name}(#{args_pass})#{if async.has(cmd) then '.await' else ''}?"
          if rt
            txt += "Reply::#{enum_name(rt)}(#{call}),"
          else
            txt+= "{\n#{space}  #{call};\n#{space}  Reply::Undefined\n#{space}}"
          li.push txt
        space+li.join('\n'+space)
    )

    modify(
      'wasm/src/w.rs'
      (txt)=>
        li = []

        for [name, args, _, func] from api_cmd
          args_pass = args.map((i)=>i[0]).join(', ')
          if args.length
            args_pass = "(#{args_pass})"
          args = args.map((x)=>x.join(': '))
          args.unshift('&mut self')
          args = args.join(', ')
          li.push(
            """\n  pub fn #{func}(#{args}) -> Promise {\n    self.req(Cmd::#{name}#{args_pass})\n  }"""
          )
        li.sort()
        li.join('\n')
    )
    modify(
      'wasm/src/reply.rs'
      (txt)=>
        li = txt.split("(").map((i)=>i.split('::',2)[1]).filter(Boolean)
        exist = new Set(li)
        for i in api_cmd
          i = i[2]
          if i
            i = enum_name i
            if not exist.has(i)
              exist.add i
              txt += "  Reply::#{i}(r) => Ok(r.into()),\n  "
        txt
    )
    modify(
      'api/src/cmd.rs'
      (cmd)=>
        exist = cmd.split(',').map(
          (i)=>
            i.split('(',1)[0].trim()
        ).filter(Boolean)

        len = exist.length
        cmd_pos = {}
        for [key] from api_cmd
          pos = exist.indexOf(key)
          if pos < 0
            pos = len
          cmd_pos[key]=pos


        '  '+api_cmd.sort(
          (a,b)=>
            cmd_pos[a[0]] - cmd_pos[b[0]]
        ).map(
          (x)=>
            t = x[1]
            if t.length
              args='('+t.map((x)=>'\n    '+x[1]+', //'+x[0]+'\n').join('')+'  )'
            else
              args = ''
            x[0]+args
        ).join(',\n  ')+','
    )
    modify(
      'api/src/reply.rs'
      (cmd)=>
        exist = new Set(cmd.split(',').map(
          (i)=>
            i.trim()
        ).filter(Boolean))
        rt_set = new Set()

        li = []
        for i in api_cmd
          i = i[2]
          if i
            i = enum_name(i)+'('+i+')'
            if not exist.has(i)
              rt_set.add i

        rt_set = [...rt_set]
        if rt_set.length
          cmd += '  '+rt_set.join(',\n  ')+',\n'

        cmd
    )
  ]
  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
