const path = require('path')
const express = require('express')
const {requestAuthorize} = require('./handler/request_authorize')
const {callback} = require('./handler/callback')

const client = express()

client.get('/request_authorize', requestAuthorize)
client.get('/callback', callback)
client.use(express.static(path.resolve(__dirname, 'static')));

module.exports = {client}
