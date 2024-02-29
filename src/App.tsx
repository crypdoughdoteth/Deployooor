import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [returnedJson, setReturnedJson] = useState<any>();

  useEffect(() => {
    (async () => {
      const json = await invoke("db_read");
      console.log(json);
      setReturnedJson(json);
    })()
  }, [])

  return (
    <div className="container">
      <h1>Welcome to Vyper Deployooor!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Deployments:</p>

      <pre>
        <code>
          {JSON.stringify(returnedJson, null, 2)}
        </code>
      </pre>
    </div>
  );
}

export default App;
