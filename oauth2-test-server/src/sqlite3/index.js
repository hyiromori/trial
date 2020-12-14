const path = require('path')
const sqlite3 = require('sqlite3').verbose()

let database;
database = new sqlite3.Database(path.resolve(__dirname, "data.sqlite3"), () => {
  database.run(`CREATE TABLE IF NOT EXISTS authorizations (
    code PRIMARY KEY,
    request_state VARCHAR(100)
  )`)
})

const runSql = (sql) => {
  database.run(sql)
};

const runSelectRow = (sql) => new Promise((resolve, reject) => database.get(sql, (err, data) => {
  if (err) {
    reject(err)
  } else {
    resolve(data)
  }
}))

module.exports = {runSql, runSelectRow}
