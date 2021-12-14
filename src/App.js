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
    },
    ls: {
        description: 'List the contents of a directory.',
        usage: 'ls path/to/directory',
        fn: function () {
						if (arguments[0]) {
                return p.native.ls(arguments[0]);
						} else {
							return p.native.ls(p.native.pwd());
						}
        }
    },
    mkdir: {
        description: 'make a directory at specified location',
        usage: 'mkdir path/to/directory',
        fn: function () {
                return p.native.mkdir(arguments[0]);
        }
    },
    cd: {
        description: 'change location in file system',
        usage: 'cd path/to/directory',
        fn: function () {
                if (arguments[0]) {
                  return p.native.cd_with_args(arguments[0]);
                } else {
									return p.native.cd();
								}
        }
    },
		cp: {
			description: 'copy file from one location to another',
			usage: 'cp path/to/source path/to/destination',
			fn: function () {
							return p.native.cp(arguments[0], arguments[1]);
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
