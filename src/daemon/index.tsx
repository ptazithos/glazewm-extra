import React, { useEffect } from 'react';
import ReactDOM from 'react-dom/client';

import { useFocusedChange } from "./hooks/useFocusedChange";

const DaemonApp = () => {
    useFocusedChange();
    
    return (
      <></>
    );
  };


const rootEl = document.getElementById('root');
if (rootEl) {
  const root = ReactDOM.createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <DaemonApp />
    </React.StrictMode>,
  );
}
