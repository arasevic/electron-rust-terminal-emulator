import React from "react";
import logo from "./logo.svg";
import "./App.css";

const p = window.preload;

function App() {
 return (
  <div className="App">
   <header className="App-header">
    <img src={logo} className="App-logo" alt="logo" />
    <p>example output of execution of rust functions called within node</p>
    <p>{p.native.hello_from_rust()}</p>
    <ul>
     <li>p.is_dev = {p.is_dev.toString()}</li>
     <li>
      p.native.example_add( Math.PI, 1.23 ) ={" "}
      {p.native.example_add(Math.PI, 1.23)}
     </li>
     <li>
      p.native.example_add( -1.23, Math.E ) ={" "}
      {p.native.example_add(-1.23, Math.E)}
     </li>
     <li>
      p.native.example_concat( "🍣su", "shi🍣" ) ={" "}
      {p.native.example_concat("🍣su", "shi🍣")}
     </li>
     <li>
      p.native.example_concat( "🌶haba", "nero🌶" ) ={" "}
      {p.native.example_concat("🌶haba", "nero🌶")}
     </li>
     <li>
      p.native.help() ={" "}
      {p.native.help()}
     </li>
     <li>
      p.native.pwd() ={" "}
      {p.native.pwd()}
     </li>
     <li>
      p.native.cd() ={" "}
      {p.native.cd()}
     </li>
     <li>
      p.native.pwd() ={" "}
      {p.native.pwd()}
     </li>
     <li>
      p.native.cd_with_args("/Users/tesla/school") ={" "}
      {p.native.cd_with_args("/Users/tesla/school")}
     </li>
     <li>
      p.native.pwd() ={" "}
      {p.native.pwd()}
     </li>
     <li>
      p.native.mkdir("testing_creating_directory") ={" "}
      {p.native.mkdir("testing_creating_directory")}
     </li>
     <li>
      p.native.pwd() ={" "}
      {p.native.pwd()}
     </li>
    </ul>
   </header>
  </div>
 );
}

export default App;
