//import reactLogo from "./assets/react.svg";
//import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import "./App.css";

function Home() {
  return (
    <main>
      <p> Home </p>
    </main>
  );
}

function CurrentWorkout() {
  return (
    <main>
      <p> CurrentWorkout </p>
    </main>
  );
}

function Progress() {
  return (
    <main>
      <p> Progress </p>
    </main>
  );
}

function Settings() {
  return (
    <main>
      <p> Settings </p>
    </main>
  );
}

function App() {
  const [currentScreen, setCurrentScreen] = useState("home");

  const renderScreen = () => {
    switch (currentScreen) {
      case 'current_workout': return <CurrentWorkout />
      case 'progress': return <Progress />
      case 'settings': return <Settings />
      default: return <Home />
    }
  }

  return (
    <main className="container">
      <h1>Dumbell</h1>


      <div className="row">
        {renderScreen()}
      </div>

      <div className="row">
        <button onClick={() => setCurrentScreen('home')} type="submit">home</button>
        <button onClick={() => setCurrentScreen('current_workout')} type="submit">current workout</button>
        <button onClick={() => setCurrentScreen('progress')} type="submit">progress</button>
        <button onClick={() => setCurrentScreen('settings')} type="submit">settings</button>
      </div>

    </main >
  );
}

export default App;
