const path = require('path')
const express = require('express')
const {approve}= require('./handler/approve')
const {token}= require('./handler/token')

const authorizationServer = express()

authorizationServer.post('/approve', express.urlencoded(), approve)
authorizationServer.post('/token', express.json(), token)
authorizationServer.use(express.static(path.resolve(__dirname, 'static')));

module.exports = {authorizationServer}
