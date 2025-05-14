//import reactLogo from "./assets/react.svg";
//import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./App.css";

const [page, setPage] = useState("home");

function changePage()

function App() {
  return (
    <main className="container">
      <h1>Dumbell</h1>

      <p>{page}</p>
      <div className="row">
        <button type="submit">home</button>
        <button type="submit">current workout</button>
        <button type="submit">progress</button>
        <button type="submit">settings</button>
      </div>


      <div className="row">
        <p></p>
      </div>
    </main>
  );
}

export default App;
