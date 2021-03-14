const express = require('express')
const {authorizationServer} = require('./authorization_server')
const {client} = require('./client')
const {protectedResource} = require('./protected_resource')

require('./sqlite3')

const PORT = 45678
const app = express()

app.use('/authorization_server', authorizationServer)
app.use('/client', client)
app.use('/protected_resource', protectedResource)

app.get('/', (req, res) => {
  res.redirect('/client')
})

console.info(`OAuth Servers Start: http://localhost:${PORT}`)
app.listen(PORT)
