const path = require('path')
const express = require('express')
const protectedResource = express()

protectedResource.use(express.static(path.resolve(__dirname, 'static')));

module.exports = {protectedResource}
