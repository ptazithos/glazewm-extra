import React, { useEffect, useLayoutEffect } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/tauri";

import { subscribeFocusChanged } from "../ipc/subscribe";
import { getWindows } from "../ipc/command";

const DaemonApp = () => {
	useLayoutEffect(() => {
		const handle = subscribeFocusChanged(async (payload) => {
			const focused = payload?.data?.focusedContainer?.handle;

			const windows = await getWindows();
			for (const window of windows) {
				const hwnd = window?.handle;
				if (hwnd === focused) {
					invoke("set_window_alpha", { rawHandle: hwnd, alpha: 255 });
				} else {
					invoke("set_window_alpha", { rawHandle: hwnd, alpha: 220 });
				}
			}
		});

		return () => {
			handle.close();
		};
	}, []);

	return <></>;
};

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<DaemonApp />
		</React.StrictMode>,
	);
}
