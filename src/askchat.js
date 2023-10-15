import { config } from 'dotenv'
config({override:true})

import {yieldStream} from 'yield-stream'
import { OpenAI } from "openai-streams/node"

function getDeltas(ev) {
  return ev.choices[0].delta
}

async function askChat(messages, functions, model='gpt-3.5-turbo-16k') {
  let cfg = { model, messages, functions, 
              temperature: 0.05, n: 1,
              presence_penalty: 0.6 }   
  const stream = await OpenAI("chat", cfg, {mode: 'raw'} )
  let decoder = new TextDecoder()
  let content, function_call
  try {
    for await (const chunk_ of stream) {
      let chunk = JSON.parse(decoder.decode(chunk_))
      if (chunk.error) {
        throw new Error(JSON.stringify(chunk))
        return 
      }
      let deltas = getDeltas(chunk)
      for (let key in deltas) {
        if (key == 'function_call') {
          if (!function_call) function_call = {}
          for (let key2 in deltas[key]) {
            if (!function_call[key2]) function_call[key2] = ''
            function_call[key2] += deltas[key][key2]
            if (deltas[key][key2]) process.stdout.write(deltas[key][key2])
         }
        } else if (key == 'content') {
          if (!content) content = ''
          content += deltas.content
          if (deltas.content) process.stdout.write(deltas.content)
        }
      }
    }
    console.log()
    return {content, function_call}
  } catch (e) {
    console.error(e)
    throw e
  }
}

export default askChat
