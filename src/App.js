import React from "react";
import Terminal from 'react-console-emulator';
import logo from "./logo.svg";
import "./App.css";

const p = window.preload;

const commands = {
    echo: {
      description: 'Echo a passed string.',
      usage: 'echo <string>',
      fn: function () {
        return `${Array.from(arguments).join(' ')}`
      }
    },
    pwd: {
			description: 'Print the present working directory.',
			usage: 'pwd',
			fn: function () {
					return p.native.pwd();
			}
    }
  }

function App() {
 return (
    <Terminal
    commands={commands}
    welcomeMessage={'Welcome to the electron-rust terminal!'}
    promptLabel={'testudo:~$'}
  />
 );
}

export default App;
