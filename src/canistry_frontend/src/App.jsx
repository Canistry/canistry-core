import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as canistry_backend_idl, canisterId as canistry_backend_id } from 'dfx-generated/canistry_backend';

const agent = new HttpAgent();
const canistryBackend = Actor.createActor(canistry_backend_idl, { agent, canisterId: canistry_backend_id });

function App() {
  const [deployMessage, setDeployMessage] = useState('');
  const [metrics, setMetrics] = useState('');
  const [alertMessage, setAlertMessage] = useState('');

  const deployCanister = async () => {
    const canisterCode = { code: new Uint8Array([0, 1, 2, 3]) }; // Replace with actual canister code
    const result = await canistryBackend.deploy_canister(canisterCode);
    setDeployMessage(result);
  };

  const getMetrics = async () => {
    const result = await canistryBackend.get_canister_metrics('bd3sg-teaaa-aaaaa-qaaba-cai');
    setMetrics(result);
  };

  const setAlert = async () => {
    const result = await canistryBackend.set_alert('bd3sg-teaaa-aaaaa-qaaba-cai', 100);
    setAlertMessage(result);
  };

  return (
    <div>
      <h1>Canistry</h1>
      <button type="button" onClick={deployCanister}>Deploy Canister</button>
      <p>{deployMessage}</p>
      <button type="button" onClick={getMetrics}>Get Canister Metrics</button>
      <p>{metrics}</p>
      <button type="button" onClick={setAlert}>Set Alert</button>
      <p>{alertMessage}</p>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById('root'));
