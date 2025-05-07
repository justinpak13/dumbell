import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [state, setState] = useState("dog");

  const [list, setList] = useState([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("shit", { name }));
  }

  async function getState() {
    invoke("change_state")
  }

  async function changeState() {
    setState(await invoke("get_state", {state}))
  }

  async function set_list() {
    setList(await invoke("get_list", {list}))
  }

  let content;
  if (state == "Rock") {
    content = <p>"do you smell"</p>
  } else {
    content = <p>"not rock"</p>
  }

  set_list();

  const listItems = list.map(item =>
    <li key={item}>
      {item}
    </li>
  );

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
          changeState();
          getState();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <p>{state}</p>
      {content}
      {listItems}




      <div className="row">
        <p></p>
      </div>
    </main>
  );
}

export default App;
