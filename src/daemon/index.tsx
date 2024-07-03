import React, { useEffect, useLayoutEffect } from 'react';
import ReactDOM from 'react-dom/client';
import {invoke} from "@tauri-apps/api/tauri"

import type { FocusChangedPayload } from '../network/subscribe';
import { subscribe } from '../network/subscribe';




const DaemonApp = () => {
    useLayoutEffect(()=>{
      const handle = subscribe<FocusChangedPayload>('focus_changed', (payload) => {
        const hwnd = payload?.data?.focusedContainer?.handle;
        invoke("set_window_alpha", {rawHandle:hwnd,alpha: 220})
      })

      return ()=>{
        handle.close();
      }
    },[])
    
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
