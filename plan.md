# System Overview

## Main Components

Caddy Server
  TLS for everything 

Front End Website (agentnexus.space)
 - landing page
 - pricing

Admin web application (agentnexus.space/app)
 - edit agents/apps

Agent host web application (nameXYZ.agentnexus.space)
 Rust
 - run agents as chatbots
 - or Discord bots

Agents (chatbots)
 Rhai
 - stateful, multistage, actions
 - db and vector db access

