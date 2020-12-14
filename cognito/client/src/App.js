import React, { useState } from 'react'
import './App.css'
import { CognitoUserPool } from 'amazon-cognito-identity-js'

// https://github.com/aws-amplify/amplify-js/tree/master/packages/amazon-cognito-identity-js
const UserPool = new CognitoUserPool({
  UserPoolId: 'ap-northeast-1_VigxoClGc',
  ClientId: '21l92js7dqsrse83v2rqf9kbpb',
})

function App () {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')

  const signUp = () => {
    UserPool.signUp(
      email,
      password,
      [],
      null,
      (err, result) => {
        if (err) {
          alert(err.message || JSON.stringify(err))
          return
        }
        const cognitoUser = result.user
        console.log(cognitoUser)
        console.log('user name is ' + cognitoUser.getUsername())
      },
    )
  }

  const signIn = () => {}

  return (
    <div className="App">
      <form>
        <table>
          <tbody>
            <tr>
              <th>Email</th>
              <td>
                <input
                  type="text"
                  value={email}
                  onChange={(event) => {
                    event.stopPropagation()
                    setEmail(event.target.value)
                  }}
                />
              </td>
            </tr>
            <tr>
              <th>Password</th>
              <td>
                <input
                  type="password"
                  value={password}
                  onChange={(event) => {
                    event.stopPropagation()
                    setPassword(event.target.value)
                  }}
                  autoComplete="off"
                />
              </td>
            </tr>
            <tr>
              <th>
                <button onClick={signUp}>Sign Up</button>
              </th>
              <th>
                <button onClick={signIn}>Sign In</button>
              </th>
            </tr>
          </tbody>
        </table>
      </form>
    </div>
  )
}

export default App
