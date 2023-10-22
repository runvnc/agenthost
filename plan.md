# Plan

- each user has their own dir
  with conversation

- chat sessions are persistent

- can run different agents
  - each agent has their own dir with scripts


- refactor agent as Impl

- decouple runner from agent host core
  runs as daemon


  - list agents

  - list users

  - HTTP API

  - create session event (agent, user)

  - input event (sessionid = agent,user)
    server-side stream events back


- cli runner talks to server
  

- should be able to optionally run continuously
  - up to X tokens or messages or something


